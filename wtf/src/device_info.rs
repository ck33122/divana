use winapi::shared::basetsd::INT_PTR;
use {
  std::{
    alloc::{alloc, alloc_zeroed, dealloc, Layout},
    fmt,
    mem::{size_of, zeroed},
    pin::Pin,
    sync::mpsc,
    thread,
  },
  thiserror::Error,
  winapi::{
    shared::{
      basetsd::{DWORD_PTR, UINT_PTR},
      minwindef::{DWORD, UINT},
      mmreg::{WAVEFORMATEX, WAVE_FORMAT_PCM},
    },
    um::{
      mmeapi::{
        waveInAddBuffer, waveInClose, waveInGetDevCapsW, waveInGetNumDevs, waveInOpen, waveInPrepareHeader, waveInReset, waveInStart,
        waveInStop, waveInUnprepareHeader,
      },
      mmsystem::*,
    },
  },
};

const WHDR_DONE: DWORD = 0x00000001; /* done bit */
const WHDR_PREPARED: DWORD = 0x00000002; /* set if this header has been prepared */
const WHDR_BEGINLOOP: DWORD = 0x00000004; /* loop start block */
const WHDR_ENDLOOP: DWORD = 0x00000008; /* loop end block */
const WHDR_INQUEUE: DWORD = 0x00000010; /* reserved for driver */

fn whdr_to_str(whdr: DWORD) -> String {
  let mut res = vec![];
  if whdr & WHDR_DONE != 0 {
    res.push("WHDR_DONE")
  };
  if whdr & WHDR_PREPARED != 0 {
    res.push("WHDR_PREPARED")
  };
  if whdr & WHDR_BEGINLOOP != 0 {
    res.push("WHDR_BEGINLOOP")
  };
  if whdr & WHDR_ENDLOOP != 0 {
    res.push("WHDR_ENDLOOP")
  };
  if whdr & WHDR_INQUEUE != 0 {
    res.push("WHDR_INQUEUE")
  };
  res.join(",")
}

#[derive(Clone, Copy)]
pub struct DeviceFormat {
  pub format: DWORD,
  pub frequency: u32,
  pub channels: u16,
  pub bits: u16,
}

#[derive(Error, Debug, Clone)]
pub enum OpenDeviceError {
  #[error("winapi mm error {code:?}")]
  MultimediaError { code: u32, description: &'static str },
  #[error("unknown error")]
  UnknownError,
}

#[derive(Clone)]
pub struct DeviceInfo {
  pub index: u32,
  name: String,
  formats: Vec<DeviceFormat>,
}

pub struct InputDevice {
  handle: HWAVEIN,
  header: WAVEHDR,
  buffer: *mut i8,
  buffer_size: usize,
  format: DeviceFormat,
  sender: mpsc::Sender<SenderSignal>,
  pub test: &'static str,
}

pub type InputDevicePtr = Pin<Box<InputDevice>>;

struct InputDeviceRawPtr(*mut InputDevice);
unsafe impl Send for InputDeviceRawPtr {}
unsafe impl Sync for InputDeviceRawPtr {}

pub enum SenderSignal {
  Init,
  Stop,
  Start,
  NewData,
}

pub struct SenderThread {
  sender: mpsc::Sender<SenderSignal>,
  thread: Option<std::thread::JoinHandle<()>>,
}

impl SenderThread {
  pub unsafe fn new(
    input_device_ptr: *mut InputDevice,
    sender: mpsc::Sender<SenderSignal>,
    reciever: mpsc::Receiver<SenderSignal>,
  ) -> SenderThread {
    let input_device_ptr = InputDeviceRawPtr(input_device_ptr);
    let thread = thread::spawn(move || loop {
      let ref mut input_device = *input_device_ptr.0;
      let msg = match reciever.recv() {
        Ok(data) => data,
        Err(err) => {
          println!("SenderThread: recv error {}", err);
          continue;
        }
      };
      match msg {
        SenderSignal::Init => println!("SenderThread: initialized!"),
        SenderSignal::Stop => {
          println!("SenderThread: stop!");
          return;
        }
        SenderSignal::Start => println!("SenderThread: start"),
        SenderSignal::NewData => {
          if input_device.header.dwFlags & WHDR_DONE != 0 {
            // see https://docs.rs/winapi/0.3.8/i686-pc-windows-msvc/winapi/um/mmsystem/struct.WAVEHDR.html
            // (*input_device_ptr).header.lpData: *mut i8
            // (*input_device_ptr).header.dwBufferLength: u32
            let mmresult = waveInPrepareHeader(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
            if mmresult != MMSYSERR_NOERROR {
              println!("waveInPrepareHeader error {}", mm_error_to_string(mmresult));
            } else {
              println!("waveInAddBuffer");
              let mmresult = waveInAddBuffer(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
              if mmresult != MMSYSERR_NOERROR {
                println!("waveInAddBuffer error {}", mm_error_to_string(mmresult));
              };
            };
          } else {
            println!("WARN: header.dwFlags = {} not handled!", whdr_to_str(input_device.header.dwFlags));
          }
          println!("SenderThread: new data");
        }
      }
    });
    sender.send(SenderSignal::Init).unwrap();
    SenderThread {
      sender,
      thread: Some(thread),
    }
  }
  pub fn stop(&mut self) {
    // self.sender.send(SenderSignal::Stop).unwrap();
    println!("SenderThread: joining!");
    self.thread.take().unwrap().join().unwrap();
    println!("SenderThread: joining done");
  }
}

// TODO this drop is wery connected (depends on) to SenderThread stop method, so is it good idea to split it that way?
//      may be is good idea to make abstraction based on InputDevice and SenderThread and return it to user
impl Drop for InputDevice {
  fn drop(&mut self) {
    println!("MOVE INPUT DEVICE");
    unsafe {
      println!("waveInStop");
      let mmresult = waveInStop(self.handle);
      if mmresult != MMSYSERR_NOERROR {
        panic!("waveInStop: {}", mm_error_to_string(mmresult));
      };

      println!("waveInReset");
      let mmresult = waveInReset(self.handle);
      if mmresult != MMSYSERR_NOERROR {
        panic!("waveInReset: {}", mm_error_to_string(mmresult));
      };

      println!("waveInUnprepareHeader");
      let mmresult = waveInUnprepareHeader(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
      if mmresult != MMSYSERR_NOERROR {
        panic!("waveInUnprepareHeader: {}", mm_error_to_string(mmresult));
      };

      println!("waveInClose");
      let mmresult = waveInClose(self.handle);
      if mmresult != MMSYSERR_NOERROR {
        panic!("waveInClose: {}", mm_error_to_string(mmresult));
      };
    }
    println!("drop for InputDevice done");
  }
}

impl InputDevice {
  pub fn new(buffer_size: usize, device_format: DeviceFormat, sender: mpsc::Sender<SenderSignal>) -> InputDevicePtr {
    unsafe {
      let buffer_layout = match Layout::array::<i8>(buffer_size) {
        Ok(layout) => layout,
        Err(_) => panic!("cannot determine buffer layout for InputDevice.buffer"),
      };
      Box::pin(InputDevice {
        handle: zeroed::<HWAVEIN>(),
        header: zeroed::<WAVEHDR>(),
        buffer: alloc_zeroed(buffer_layout) as *mut i8,
        buffer_size,
        sender,
        format: device_format,
        test: "helo world",
      })
    }
  }
}

impl DeviceFormat {
  pub fn unpack(packed_format: DWORD) -> Vec<DeviceFormat> {
    // TODO there is simplification: stereo not allowed, need to break this simplification in feauture
    // on_device_format: ($const_dword: expr, $frequency: expr, $channels: expr, $bits: expr)
    macro_rules! enumerate_device_formats {
      ($on_device_format: ident) => {
        $on_device_format!(WAVE_FORMAT_1M08, 11025u32, 1u16, 8u16);
        $on_device_format!(WAVE_FORMAT_1M16, 11025u32, 1u16, 16u16);
        // $on_device_format!(WAVE_FORMAT_1S08, 11025u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_1S16, 11025u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_2M08, 22050u32, 1u16, 8u16);
        $on_device_format!(WAVE_FORMAT_2M16, 22050u32, 1u16, 16u16);
        // $on_device_format!(WAVE_FORMAT_2S08, 22050u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_2S16, 22050u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_4M08, 44100u32, 1u16, 8u16);
        $on_device_format!(WAVE_FORMAT_4M16, 44100u32, 1u16, 16u16);
        // $on_device_format!(WAVE_FORMAT_4S08, 44100u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_4S16, 44100u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_96M08, 96000u32, 1u16, 8u16);
        $on_device_format!(WAVE_FORMAT_96M16, 96000u32, 1u16, 16u16);
        // $on_device_format!(WAVE_FORMAT_96S08, 96000u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_96S16, 96000u32, 2u16, 16u16)
      };
    }
    let mut result: Vec<DeviceFormat> = Vec::new();
    macro_rules! expand_device_format_enum {
      ($const_dword: expr, $frequency: expr, $channels: expr, $bits: expr) => {
        if packed_format & $const_dword != 0 {
          result.push(DeviceFormat {
            format: $const_dword,
            frequency: $frequency,
            channels: $channels,
            bits: $bits,
          })
        }
      };
    }
    enumerate_device_formats!(expand_device_format_enum);
    result
  }
  pub fn relevant_frequencies() -> [u32; 4] {
    [44100, 96000, 22050, 11025]
  }
}

impl DeviceInfo {
  pub fn input_devices() -> Vec<DeviceInfo> {
    let mut available_devices: Vec<DeviceInfo> = Vec::new();
    unsafe {
      let device_count = waveInGetNumDevs();
      for device_index in 0..device_count {
        let size = size_of::<WAVEINCAPSW>() as u32;
        let mut device_capabilities = zeroed::<WAVEINCAPSW>();
        if waveInGetDevCapsW(device_index as UINT_PTR, &mut device_capabilities, size) != MMSYSERR_NOERROR {
          continue;
        }
        let name = match String::from_utf16(&device_capabilities.szPname) {
          Ok(res) => res,
          _ => continue,
        };
        let formats = DeviceFormat::unpack(device_capabilities.dwFormats);
        let info = DeviceInfo {
          index: device_index,
          name,
          formats,
        };
        if info.formats.is_empty() {
          continue;
        }
        available_devices.push(info);
      }
    }
    available_devices
  }

  pub fn open_input_stream(requested_format: DeviceFormat, device_index: u32) -> Result<(InputDevicePtr, SenderThread), OpenDeviceError> {
    unsafe {
      let mut format = zeroed::<WAVEFORMATEX>();
      format.wFormatTag = WAVE_FORMAT_PCM;
      format.nChannels = requested_format.channels;
      format.nSamplesPerSec = requested_format.frequency; // assumes that channels = 1
      format.wBitsPerSample = requested_format.bits;
      format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
      format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
      format.cbSize = 0;
      let (sender, reciever) = mpsc::channel();
      let mut input_device = InputDevice::new(format.nAvgBytesPerSec as usize, requested_format, sender.clone());
      let input_device_ptr: *mut InputDevice = input_device.as_mut().get_unchecked_mut();
      let sender_thread = SenderThread::new(input_device_ptr, sender.clone(), reciever);

      let mmresult = waveInOpen(
        &mut input_device.handle,
        device_index,
        &format,
        wave_in_callback as DWORD_PTR,
        input_device_ptr as DWORD_PTR,
        CALLBACK_FUNCTION, // | WAVE_FORMAT_DIRECT??? does not perform conversions on the audio data
      );
      if mmresult != MMSYSERR_NOERROR {
        return Err(OpenDeviceError::MultimediaError {
          code: mmresult,
          description: mm_error_to_string(mmresult),
        });
      };

      input_device.header.lpData = input_device.buffer;
      input_device.header.dwBufferLength = input_device.buffer_size as u32;
      let mmresult = waveInPrepareHeader(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
      if mmresult != MMSYSERR_NOERROR {
        return Err(OpenDeviceError::MultimediaError {
          code: mmresult,
          description: mm_error_to_string(mmresult),
        });
      };

      let mmresult = waveInAddBuffer(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
      if mmresult != MMSYSERR_NOERROR {
        return Err(OpenDeviceError::MultimediaError {
          code: mmresult,
          description: mm_error_to_string(mmresult),
        });
      };

      println!("running input");
      let mmresult = waveInStart(input_device.handle);
      if mmresult != MMSYSERR_NOERROR {
        return Err(OpenDeviceError::MultimediaError {
          code: mmresult,
          description: mm_error_to_string(mmresult),
        });
      };

      Ok((input_device, sender_thread))
    }
  }

  pub fn get_best_format(&self) -> DeviceFormat {
    let frequencies = DeviceFormat::relevant_frequencies();
    for &frequency in &frequencies {
      let mut freq_formats: Vec<&DeviceFormat> = self.formats.iter().filter(|x| x.frequency == frequency).collect();
      if !freq_formats.is_empty() {
        freq_formats.sort_by_key(|x| x.bits);
        freq_formats.sort_by_key(|x| x.channels);
        return freq_formats[0].clone();
      }
    }
    panic!("should not happen")
  }
}

impl fmt::Display for DeviceInfo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}
impl fmt::Display for DeviceFormat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}hz {} {}bit",
      self.frequency,
      if self.channels == 1 { "Mono" } else { "Stereo" },
      self.bits
    )
  }
}

unsafe extern "stdcall" fn wave_in_callback(
  device_handle: HWAVEIN,
  message: UINT,
  instance_data: DWORD_PTR,
  message_param_1: DWORD_PTR,
  message_param_2: DWORD_PTR,
) -> INT_PTR {
  if message != WIM_DATA && message != WIM_CLOSE && message != WIM_OPEN {
    println!("CALLBACK ERROR GOT UNKNOWN MESSAGE");
    return 0;
  }

  let instance = instance_data as *mut InputDevice;
  let ref mut input_device = *instance;

  if message == WIM_DATA {
    if input_device.header.dwFlags & WHDR_DONE != 0 {
      // see https://docs.rs/winapi/0.3.8/i686-pc-windows-msvc/winapi/um/mmsystem/struct.WAVEHDR.html
      // (*input_device_ptr).header.lpData: *mut i8
      // (*input_device_ptr).header.dwBufferLength: u32
      println!("waveInPrepareHeader");
      // TODO send buffer here?????
      // let mmresult = waveInPrepareHeader(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
      // if mmresult != MMSYSERR_NOERROR {
      //   println!("waveInPrepareHeader error {}", mm_error_to_string(mmresult));
      // } else {
      //   println!("waveInAddBuffer");
      //   let mmresult = waveInAddBuffer(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
      //   if mmresult != MMSYSERR_NOERROR {
      //     println!("waveInAddBuffer error {}", mm_error_to_string(mmresult));
      //   };
      // };
    }
  } else if message == WIM_CLOSE {
    println!("EXITING!");
    let buffer_layout = match Layout::array::<i8>(input_device.buffer_size) {
      Ok(layout) => layout,
      Err(_) => panic!("cannot determine buffer layout for InputDevice.buffer"),
    };
    dealloc(input_device.buffer as *mut u8, buffer_layout);
    return 1;
  }

  //--------------------------------------------------------------
  let msg = match message {
    WIM_CLOSE => "WIM_CLOSE",
    WIM_DATA => "WIM_DATA",
    WIM_OPEN => "WIM_OPEN",
    _ => "unknown",
  };
  println!("wave_in_callback: {} with params: {:?} {:?}", msg, message_param_1, message_param_2);
  //--------------------------------------------------------------
  // TODO unwrap here is not good idea (it gives error if sender is dropped)
  match message {
    WIM_CLOSE => (*instance).sender.send(SenderSignal::Stop).unwrap(),
    WIM_DATA => (*instance).sender.send(SenderSignal::NewData).unwrap(),
    WIM_OPEN => (*instance).sender.send(SenderSignal::Start).unwrap(),
    _ => {}
  };
  1
}

fn mm_error_to_string(r: MMRESULT) -> &'static str {
  match r {
    MMSYSERR_NOERROR => "NOERROR",
    MMSYSERR_ERROR => "ERROR",
    MMSYSERR_BADDEVICEID => "BADDEVICEID",
    MMSYSERR_NOTENABLED => "NOTENABLED",
    MMSYSERR_ALLOCATED => "ALLOCATED",
    MMSYSERR_INVALHANDLE => "INVALHANDLE",
    MMSYSERR_NODRIVER => "NODRIVER",
    MMSYSERR_NOMEM => "NOMEM",
    MMSYSERR_NOTSUPPORTED => "NOTSUPPORTED",
    MMSYSERR_BADERRNUM => "BADERRNUM",
    MMSYSERR_INVALFLAG => "INVALFLAG",
    MMSYSERR_INVALPARAM => "INVALPARAM",
    MMSYSERR_HANDLEBUSY => "HANDLEBUSY",
    MMSYSERR_INVALIDALIAS => "INVALIDALIAS",
    MMSYSERR_BADDB => "BADDB",
    MMSYSERR_KEYNOTFOUND => "KEYNOTFOUND",
    MMSYSERR_READERROR => "READERROR",
    MMSYSERR_WRITEERROR => "WRITEERROR",
    MMSYSERR_DELETEERROR => "DELETEERROR",
    MMSYSERR_VALNOTFOUND => "VALNOTFOUND",
    MMSYSERR_NODRIVERCB => "NODRIVERCB",
    MMSYSERR_MOREDATA => "MOREDATA",
    _ => "[unknown (not MM-system error)]",
  }
}
