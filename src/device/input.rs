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
      minwindef::DWORD,
      mmreg::{WAVEFORMATEX, WAVE_FORMAT_PCM},
    },
    um::{
      mmeapi::{
        waveInAddBuffer, waveInClose, waveInOpen, waveInPrepareHeader, waveInReset, waveInStart, waveInStop, waveInUnprepareHeader,
      },
      mmsystem::*,
    },
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
  fn buffer_layout(buffer_size: usize) -> Layout {
    match Layout::array::<i8>(buffer_size) {
      Ok(layout) => layout,
      Err(_) => panic!("cannot determine buffer layout"),
    }
  }

  // TODO handle errors
  pub unsafe fn new(desired_format: DeviceFormat, device_index: u32) -> InputDevicePtr {
    let mut format = zeroed::<WAVEFORMATEX>();
    format.wFormatTag = WAVE_FORMAT_PCM;
    format.nChannels = desired_format.channels;
    format.nSamplesPerSec = desired_format.frequency; // assumes that channels = 1
    format.wBitsPerSample = desired_format.bits;
    format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
    format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
    format.cbSize = 0;
    let buffer_size = format.nAvgBytesPerSec as usize;
    let buffer_layout = Self::buffer_layout(buffer_size);
    let (sender, reciever) = mpsc::channel();

    let thread = thread::spawn(move || {
      let mut handle = zeroed::<HWAVEIN>();
      let mut header = zeroed::<WAVEHDR>();
      let buffer = alloc_zeroed(buffer_layout) as *mut i8;
      loop {
        let msg = match reciever.recv_timeout(Duration::from_millis(25)) {
          Ok(msg) => msg,
          Err(RecvTimeoutError::Timeout) => SenderSignal::NewData,
          Err(err) => {
            println!("SenderThread: recv error {}", err);
            continue;
          }
        };
        match msg {
          SenderSignal::Init => {
            // WAVE_FORMAT_DIRECT??? does not perform conversions on the audio data
            let mmresult = waveInOpen(&mut handle, device_index, &format, 0 as DWORD_PTR, 0 as DWORD_PTR, 0);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInOpen: {}", mm_error_to_string(mmresult));
            };
            header.lpData = buffer;
            header.dwBufferLength = buffer_size as u32;
            let mmresult = waveInPrepareHeader(handle, &mut header, size_of::<WAVEHDR>() as u32);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInPrepareHeader: {}", mm_error_to_string(mmresult));
            };
            let mmresult = waveInAddBuffer(handle, &mut header, size_of::<WAVEHDR>() as u32);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInAddBuffer: {}", mm_error_to_string(mmresult));
            };
            println!("running input");
            let mmresult = waveInStart(handle);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInStart: {}", mm_error_to_string(mmresult));
            };
            println!("SenderThread: initialized!");
          }
          SenderSignal::NewData => {
            if header.dwFlags & WHDR_INQUEUE != 0 {
              continue;
            }
            if header.dwFlags & WHDR_DONE != 0 {
              println!("SenderThread: new data");
              // see https://docs.rs/winapi/0.3.8/i686-pc-windows-msvc/winapi/um/mmsystem/struct.WAVEHDR.html
              // header.lpData: *mut i8
              // header.dwBufferLength: u32
              let mmresult = waveInPrepareHeader(handle, &mut header, size_of::<WAVEHDR>() as u32);
              if mmresult != MMSYSERR_NOERROR {
                panic!("waveInPrepareHeader: {}", mm_error_to_string(mmresult));
              }
              let mmresult = waveInAddBuffer(handle, &mut header, size_of::<WAVEHDR>() as u32);
              if mmresult != MMSYSERR_NOERROR {
                panic!("waveInAddBuffer error {}", mm_error_to_string(mmresult));
              };
              continue;
            }
            println!("WARN: header.dwFlags = {} not handled!", whdr_to_str(header.dwFlags));
          }
          SenderSignal::Stop => {
            println!("waveInStop");
            let mmresult = waveInStop(handle);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInStop: {}", mm_error_to_string(mmresult));
            };
            println!("waveInReset");
            let mmresult = waveInReset(handle);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInReset: {}", mm_error_to_string(mmresult));
            };
            println!("waveInUnprepareHeader");
            let mmresult = waveInUnprepareHeader(handle, &mut header, size_of::<WAVEHDR>() as u32);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInUnprepareHeader: {}", mm_error_to_string(mmresult));
            };
            println!("waveInClose");
            let mmresult = waveInClose(handle);
            if mmresult != MMSYSERR_NOERROR {
              panic!("waveInClose: {}", mm_error_to_string(mmresult));
            };

            println!("SenderThread: stop!");
            dealloc(buffer as *mut u8, buffer_layout);
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
