use {
  crate::device::{common::*, info::*},
  std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    mem::{size_of, zeroed},
    pin::Pin,
    sync::{mpsc, mpsc::RecvTimeoutError},
    thread,
    time::Duration,
  },
  // thiserror::Error,
  winapi::{
    shared::{
      basetsd::DWORD_PTR,
      mmreg::{WAVEFORMATEX, WAVE_FORMAT_PCM},
    },
    um::{mmeapi::*, mmsystem::*},
  },
};

pub struct InputDevice {
  sender: mpsc::Sender<SenderSignal>,
  thread: Option<std::thread::JoinHandle<()>>,
}

pub enum SenderSignal {
  Init,
  Stop,
  NewData,
}

// #[derive(Error, Debug, Clone)]
// pub enum OpenDeviceError {
//   #[error("winapi mm error {code:?}")]
//   MultimediaError { code: u32, description: &'static str },
//   #[error("unknown error")]
//   UnknownError,
// }

pub type InputDevicePtr = Pin<Box<InputDevice>>;

impl Drop for InputDevice {
  fn drop(&mut self) {
    println!("InputDevice.drop is running");
    if self.thread.is_some() {
      println!("InputDevice.drop: sending Stop signal");
      self.sender.send(SenderSignal::Stop).unwrap();
      println!("InputDevice.drop: running join!");
      self.thread.take().unwrap().join().unwrap();
      println!("InputDevice.drop: joining done");
    }
    println!("InputDevice.drop done");
  }
}

impl InputDevice {
  // TODO handle errors
  pub fn new(desired_format: DeviceFormat, device_index: u32) -> InputDevicePtr {
    let (sender, reciever) = mpsc::channel();
    let thread = thread::spawn(move || unsafe {
      let mut input_processor = InputProcessor::new(desired_format, device_index);
      loop {
        let msg = match reciever.recv_timeout(Duration::from_millis(10)) {
          Ok(msg) => msg,
          Err(RecvTimeoutError::Timeout) => SenderSignal::NewData,
          Err(err) => {
            println!("ThreadLoop: recv error {}", err);
            continue;
          }
        };
        match msg {
          SenderSignal::Init => input_processor.init(),
          SenderSignal::NewData => input_processor.new_data(),
          SenderSignal::Stop => {
            input_processor.stop();
            break;
          }
        }
      }
    });
    sender.send(SenderSignal::Init).unwrap();
    Box::pin(InputDevice {
      sender,
      thread: Some(thread),
    })
  }
}

struct InputProcessor {
  format: WAVEFORMATEX,
  device_index: u32,
  buffer_size: usize,
  buffer_layout: Layout,
  buffer: *mut i8,
  handle: HWAVEIN,
  header: WAVEHDR,
}

impl InputProcessor {
  unsafe fn new(desired_format: DeviceFormat, device_index: u32) -> InputProcessor {
    let mut format = zeroed::<WAVEFORMATEX>();
    format.wFormatTag = WAVE_FORMAT_PCM;
    format.nChannels = desired_format.channels;
    format.nSamplesPerSec = desired_format.frequency; // assumes that channels = 1
    format.wBitsPerSample = desired_format.bits;
    format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
    format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
    format.cbSize = 0;
    let buffer_size = format.nAvgBytesPerSec as usize;
    let buffer_layout = match Layout::array::<i8>(buffer_size) {
      Ok(layout) => layout,
      Err(err) => panic!("cannot determine buffer layout: {}", err),
    };
    let handle = zeroed::<HWAVEIN>();
    let header = zeroed::<WAVEHDR>();
    let buffer = alloc_zeroed(buffer_layout) as *mut i8;
    InputProcessor {
      format,
      buffer_size,
      buffer_layout,
      handle,
      header,
      buffer,
      device_index,
    }
  }

  unsafe fn init(&mut self) {
    // WAVE_FORMAT_DIRECT??? does not perform conversions on the audio data
    let mmresult = waveInOpen(&mut self.handle, self.device_index, &self.format, 0 as DWORD_PTR, 0 as DWORD_PTR, 0);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveInOpen: {}", mm_error_to_string(mmresult));
    };
    self.header.lpData = self.buffer;
    self.header.dwBufferLength = self.buffer_size as u32;
    let mmresult = waveInPrepareHeader(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveInPrepareHeader: {}", mm_error_to_string(mmresult));
    };
    let mmresult = waveInAddBuffer(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveInAddBuffer: {}", mm_error_to_string(mmresult));
    };
    println!("running input");
    let mmresult = waveInStart(self.handle);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveInStart: {}", mm_error_to_string(mmresult));
    };
    println!("SenderThread: initialized!");
  }

  unsafe fn new_data(&mut self) {
    if self.header.dwFlags & WHDR_INQUEUE != 0 {
      return;
    }
    if self.header.dwFlags & WHDR_DONE != 0 {
      println!("SenderThread: new data");
      // see https://docs.rs/winapi/0.3.8/i686-pc-windows-msvc/winapi/um/mmsystem/struct.WAVEHDR.html
      // header.lpData: *mut i8
      // header.dwBufferLength: u32
      //
      // let mmresult = waveInPrepareHeader(handle, &mut header, size_of::<WAVEHDR>() as u32);
      // if mmresult != MMSYSERR_NOERROR {
      //   panic!("waveInPrepareHeader: {}", mm_error_to_string(mmresult));
      // }
      let mmresult = waveInAddBuffer(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
      if mmresult != MMSYSERR_NOERROR {
        panic!("waveInAddBuffer error {}", mm_error_to_string(mmresult));
      };
      return;
    }
    println!("WARN: header.dwFlags = {} not handled!", whdr_to_str(self.header.dwFlags));
  }

  unsafe fn stop(&mut self) {
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

    println!("SenderThread: stop!");
    dealloc(self.buffer as *mut u8, self.buffer_layout);
  }
}
