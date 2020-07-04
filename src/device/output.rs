use {
  crate::device::{common::*, info::*},
  std::{
    mem::{size_of, zeroed},
    pin::Pin,
    sync::mpsc,
    thread,
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

pub struct OutputDevice {
  pub sender: mpsc::Sender<Command>,
  thread: Option<std::thread::JoinHandle<()>>,
}

pub enum Command {
  Init,
  Stop,
  NewData(WaveBuffer),
}

// #[derive(Error, Debug, Clone)]
// pub enum OpenDeviceError {
//   #[error("winapi mm error {code:?}")]
//   MultimediaError { code: u32, description: &'static str },
//   #[error("unknown error")]
//   UnknownError,
// }

impl Drop for OutputDevice {
  fn drop(&mut self) {
    println!("OutputDevice.drop is running");
    if self.thread.is_some() {
      println!("OutputDevice.drop: sending Stop signal");
      self.sender.send(Command::Stop).unwrap();
      println!("OutputDevice.drop: running join!");
      self.thread.take().unwrap().join().unwrap();
      println!("OutputDevice.drop: joining done");
    }
    println!("OutputDevice.drop done");
  }
}

impl OutputDevice {
  // TODO handle errors
  pub fn new(desired_format: DeviceFormat, device_index: u32) -> OutputDevice {
    let (sender, reciever) = mpsc::channel::<Command>();
    let thread = thread::Builder::new()
      .name("output".into())
      .spawn(move || unsafe {
        let mut output_processor = OutputProcessor::new(desired_format, device_index);
        loop {
          let msg = match reciever.recv() {
            Ok(msg) => msg,
            Err(err) => {
              println!("OutputDevice: recv error {}", err);
              continue;
            }
          };
          match msg {
            Command::Init => output_processor.init(),
            Command::NewData(buffer) => output_processor.new_data(buffer),
            Command::Stop => {
              output_processor.stop();
              break;
            }
          }
        }
      })
      .unwrap();
    sender.send(Command::Init).unwrap();
    OutputDevice {
      sender,
      thread: Some(thread),
    }
  }
}

struct OutputProcessor {
  format: WAVEFORMATEX,
  device_index: u32,
  handle: HWAVEOUT,
}

impl OutputProcessor {
  unsafe fn new(desired_format: DeviceFormat, device_index: u32) -> OutputProcessor {
    let mut format = zeroed::<WAVEFORMATEX>();
    format.wFormatTag = WAVE_FORMAT_PCM;
    format.nChannels = desired_format.channels;
    format.nSamplesPerSec = desired_format.frequency; // assumes that channels = 1
    format.wBitsPerSample = desired_format.bits;
    format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
    format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
    format.cbSize = 0;
    let handle = zeroed::<HWAVEOUT>();
    OutputProcessor {
      format,
      handle,
      device_index,
    }
  }

  unsafe fn init(&mut self) {
    // WAVE_FORMAT_DIRECT??? does not perform conversions on the audio data
    let mmresult = waveOutOpen(&mut self.handle, self.device_index, &self.format, 0 as DWORD_PTR, 0 as DWORD_PTR, 0);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveOutOpen: {}", mm_error_to_string(mmresult));
    };
    println!("OutputProcessor: initialized!");
  }

  unsafe fn new_data(&mut self, buffer: WaveBuffer) {
    // TODO wait device using events
    let mut header = zeroed::<WAVEHDR>();
    header.lpData = buffer.data;
    header.dwBufferLength = buffer.length();
    let mmresult = waveOutPrepareHeader(self.handle, &mut header, size_of::<WAVEHDR>() as u32);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveOutPrepareHeader: {}", mm_error_to_string(mmresult));
    };
    let mmresult = waveOutWrite(self.handle, &mut header, size_of::<WAVEHDR>() as u32);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveOutWrite error {}", mm_error_to_string(mmresult));
    };
    return;
    // if self.header.dwFlags & WHDR_DONE != 0 {
    //   println!("OutputProcessor: new data");
    //   // see https://docs.rs/winapi/0.3.8/i686-pc-windows-msvc/winapi/um/mmsystem/struct.WAVEHDR.html
    //   // header.lpData: *mut i8
    //   // header.dwBufferLength: u32
    //   //
    //   // let mmresult = waveOutPrepareHeader(handle, &mut header, size_of::<WAVEHDR>() as u32);
    //   // if mmresult != MMSYSERR_NOERROR {
    //   //   panic!("waveOutPrepareHeader: {}", mm_error_to_string(mmresult));
    //   // }
    //   // let mmresult = waveOutAddBuffer(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
    //   // if mmresult != MMSYSERR_NOERROR {
    //   //   panic!("waveOutAddBuffer error {}", mm_error_to_string(mmresult));
    //   // };
    //   return;
    // }
    //
    // println!("WARN: output header.dwFlags = {} not handled!", whdr_to_str(self.header.dwFlags));
  }

  unsafe fn stop(&mut self) {
    // TODO how to unprepare header if it is not presented here?
    println!("waveOutReset");
    let mmresult = waveOutReset(self.handle);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveOutReset: {}", mm_error_to_string(mmresult));
    };
    // println!("waveOutUnprepareHeader");
    // let mmresult = waveOutUnprepareHeader(self.handle, &mut self.header, size_of::<WAVEHDR>() as u32);
    // if mmresult != MMSYSERR_NOERROR {
    //   panic!("waveOutUnprepareHeader: {}", mm_error_to_string(mmresult));
    // };
    println!("waveOutClose");
    let mmresult = waveOutClose(self.handle);
    if mmresult != MMSYSERR_NOERROR {
      panic!("waveOutClose: {}", mm_error_to_string(mmresult));
    };
    println!("OutputProcessor: stop!");
  }
}
