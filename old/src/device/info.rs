use {
  crate::device::common::*,
  std::{
    fmt,
    mem::{size_of, zeroed},
  },
  winapi::{
    shared::{basetsd::UINT_PTR, minwindef::DWORD},
    um::{mmeapi::*, mmsystem::*},
  },
};

#[derive(Clone, Copy)]
pub struct DeviceFormat {
  pub format: DWORD,
  pub frequency: u32,
  pub channels: u16,
  pub bits: u16,
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

#[derive(Clone)]
pub struct DeviceInfo {
  pub index: u32,
  name: String,
  formats: Vec<DeviceFormat>,
}

impl fmt::Display for DeviceInfo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
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
        let mmresult = waveInGetDevCapsW(device_index as UINT_PTR, &mut device_capabilities, size);
        if mmresult != MMSYSERR_NOERROR {
          println!("waveInGetDevCapsW: {}", mm_error_to_string(mmresult));
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

  pub fn output_devices() -> Vec<DeviceInfo> {
    let mut available_devices: Vec<DeviceInfo> = Vec::new();
    unsafe {
      let device_count = waveOutGetNumDevs();
      for device_index in 0..device_count {
        let size = size_of::<WAVEOUTCAPSW>() as u32;
        let mut device_capabilities = zeroed::<WAVEOUTCAPSW>();
        let mmresult = waveOutGetDevCapsW(device_index as UINT_PTR, &mut device_capabilities, size);
        if mmresult != MMSYSERR_NOERROR {
          println!("waveOutGetDevCapsW: {}", mm_error_to_string(mmresult));
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
