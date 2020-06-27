use {
  std::{
    fmt,
    mem::{size_of, zeroed},
  },
  thiserror::Error,
  winapi::{
    shared::{
      basetsd::{DWORD_PTR, UINT_PTR},
      minwindef::{DWORD, UINT},
      mmreg::{WAVEFORMATEX, WAVE_FORMAT_PCM},
    },
    um::{
      mmeapi::{waveInGetDevCapsW, waveInGetNumDevs, waveInOpen},
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
  #[error("waveInOpen: specified resource is already allocated")]
  WaveInOpenAlreadyAllocated,
  #[error("waveInOpen: device id is out of range (possible fix: select new device)")]
  WaveInOpenBadDeviceId,
  #[error("waveInOpen: no driver available for this device (possible fix: select new device)")]
  WaveInOpenNoDriver,
  #[error("waveInOpen: no memory available")]
  WaveInOpenNoMem,
  #[error("waveInOpen: attempted to open with an unsupported waveform-audio format")]
  WaveInOpenBadFormat,
  #[error("waveInOpen: unknown error {code:?}")]
  WaveInOpenUnknownError { code: u32 },
}

#[derive(Clone)]
pub struct DeviceInfo {
  pub index: u32,
  name: String,
  formats: Vec<DeviceFormat>,
}

impl DeviceFormat {
  pub fn unpack(packed_format: DWORD) -> Vec<DeviceFormat> {
    // TODO there is simplification: stereo not allowed, need to break this simplification in feauture
    // on_device_format: ($const_dword: expr, $frequency: expr, $channels: expr, $bits: expr)
    macro_rules! enumerate_device_formats {
      ($on_device_format: ident) => {
        $on_device_format!(WAVE_FORMAT_1M08, 11025u32, 1u16, 8u16)
        $on_device_format!(WAVE_FORMAT_1M16, 11025u32, 1u16, 16u16)
        // $on_device_format!(WAVE_FORMAT_1S08, 11025u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_1S16, 11025u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_2M08, 22050u32, 1u16, 8u16)
        $on_device_format!(WAVE_FORMAT_2M16, 22050u32, 1u16, 16u16)
        // $on_device_format!(WAVE_FORMAT_2S08, 22050u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_2S16, 22050u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_4M08, 44100u32, 1u16, 8u16)
        $on_device_format!(WAVE_FORMAT_4M16, 44100u32, 1u16, 16u16)
        // $on_device_format!(WAVE_FORMAT_4S08, 44100u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_4S16, 44100u32, 2u16, 16u16)
        $on_device_format!(WAVE_FORMAT_96M08, 96000u32, 1u16, 8u16)
        $on_device_format!(WAVE_FORMAT_96M16, 96000u32, 1u16, 16u16)
        // $on_device_format!(WAVE_FORMAT_96S08, 96000u32, 2u16, 8u16)
        // $on_device_format!(WAVE_FORMAT_96S16, 96000u32, 2u16, 16u16)
      }
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

  pub fn open_input_stream(requested_format: DeviceFormat, device_index: u32) -> Option<OpenDeviceError> {
    unsafe {
      let mut format = zeroed::<WAVEFORMATEX>();
      format.wFormatTag = WAVE_FORMAT_PCM;
      format.nChannels = requested_format.channels;
      format.nSamplesPerSec = requested_format.frequency; // assumes that channels = 1
      format.wBitsPerSample = requested_format.bits;
      format.nBlockAlign = (format.wBitsPerSample / 8) * format.nChannels; // idk what is that
      format.nAvgBytesPerSec = format.nSamplesPerSec * format.nBlockAlign as u32;
      format.cbSize = 0;

      let mut in_handle = zeroed::<HWAVEIN>();
      match waveInOpen(
        &mut in_handle,
        device_index,
        &format,
        wave_in_callback as DWORD_PTR, // callback
        0,                             // callback argument
        CALLBACK_FUNCTION,             // | WAVE_FORMAT_DIRECT??? does not perform conversions on the audio data
      ) {
        MMSYSERR_NOERROR => { /* ok */ }
        MMSYSERR_ALLOCATED => return Some(OpenDeviceError::WaveInOpenAlreadyAllocated),
        MMSYSERR_BADDEVICEID => return Some(OpenDeviceError::WaveInOpenBadDeviceId),
        MMSYSERR_NODRIVER => return Some(OpenDeviceError::WaveInOpenNoDriver),
        MMSYSERR_NOMEM => return Some(OpenDeviceError::WaveInOpenNoMem),
        WAVERR_BADFORMAT => return Some(OpenDeviceError::WaveInOpenBadFormat),
        code => return Some(OpenDeviceError::WaveInOpenUnknownError { code }),
      };
      None
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

fn wave_in_callback(
  device_handle: HWAVEIN,
  message: UINT,
  instance_data: DWORD_PTR,
  message_param_1: DWORD_PTR,
  message_param_2: DWORD_PTR,
) {
  match message {
    WIM_CLOSE => println!("msg: device is closed using the waveInClose function"),
    WIM_DATA => println!("msg: device driver is finished with a data block sent using the waveInAddBuffer function"),
    WIM_OPEN => println!("msg: device is opened using the waveInOpen function"),
    _ => {}
  }
  println!(
    "{:?} {:?} {:?} {:?} {:?}",
    device_handle, message, instance_data, message_param_1, message_param_2
  );
}
