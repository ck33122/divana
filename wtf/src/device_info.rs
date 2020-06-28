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
      mmeapi::{waveInAddBuffer, waveInGetDevCapsW, waveInGetNumDevs, waveInOpen, waveInPrepareHeader, waveInStart},
      mmsystem::*,
    },
  },
};

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
  pub fn new() -> SenderThread {
    let (sender, reciever) = mpsc::channel();
    let thread = thread::spawn(move || loop {
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
        SenderSignal::NewData => println!("SenderThread: new data"),
      }
    });
    sender.send(SenderSignal::Init).unwrap();
    SenderThread {
      sender,
      thread: Some(thread),
    }
  }
  pub fn stop(&mut self) {
    self.sender.send(SenderSignal::Stop).unwrap();
    self.thread.take().unwrap().join().unwrap();
  }
}

// TODO may be call waveInStop around here?
impl Drop for InputDevice {
  fn drop(&mut self) {
    println!("MOVE INPUT DEVICE");
    unsafe {
      let buffer_layout = match Layout::array::<i8>(self.buffer_size) {
        Ok(layout) => layout,
        Err(_) => panic!("cannot determine buffer layout for InputDevice.buffer"),
      };
      dealloc(self.buffer as *mut u8, buffer_layout);
    }
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

  pub fn open_input_stream(
    requested_format: DeviceFormat,
    device_index: u32,
    sender: &SenderThread,
  ) -> Result<InputDevicePtr, OpenDeviceError> {
    unsafe {
      let mut format = zeroed::<WAVEFORMATEX>();
      format.wFormatTag = WAVE_FORMAT_PCM;
      format.nChannels = requested_format.channels;
      format.nSamplesPerSec = requested_format.frequency; // assumes that channels = 1
      format.wBitsPerSample = requested_format.bits;
      format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
      format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
      format.cbSize = 0;
      let mut input_device = InputDevice::new(format.nAvgBytesPerSec as usize, requested_format, sender.sender.clone());
      let input_device_ptr: *mut InputDevice = input_device.as_mut().get_unchecked_mut();

      println!("opening wave device");
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

      println!("preparing wave header");
      input_device.header.lpData = input_device.buffer;
      input_device.header.dwBufferLength = input_device.buffer_size as u32;
      let mmresult = waveInPrepareHeader(input_device.handle, &mut input_device.header, size_of::<WAVEHDR>() as u32);
      if mmresult != MMSYSERR_NOERROR {
        return Err(OpenDeviceError::MultimediaError {
          code: mmresult,
          description: mm_error_to_string(mmresult),
        });
      };

      println!("adding wave buffer");
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

      Ok(input_device)
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
  let instance = instance_data as *mut InputDevice;
  if instance == std::ptr::null_mut() {
    println!("error! instance_data == 0");
    return 1;
  }
  //--------------------------------------------------------------
  let msg = match message {
    WIM_CLOSE => "device is closed using the waveInClose function",
    WIM_DATA => "device driver is finished with a data block sent using the waveInAddBuffer function",
    WIM_OPEN => "device is opened using the waveInOpen function",
    _ => "unknown",
  };
  println!(
    "callback! [{}] on handle {:?} with params: {:?} {:?}",
    msg, device_handle, message_param_1, message_param_2
  );
  //--------------------------------------------------------------
  match message {
    WIM_CLOSE => (*instance).sender.send(SenderSignal::Stop).unwrap(), // TODO unwrap here is not good idea
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
