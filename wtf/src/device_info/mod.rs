mod format;

use format::*;
use std::{fmt, mem::{size_of, zeroed}};
use winapi::um::{
  mmeapi::{waveInGetDevCapsW, waveInGetNumDevs},
  mmsystem::*,
};

#[derive(Clone)]
pub struct DeviceInfo {
  index: usize,
  name: String,
  formats: Vec<DeviceFormat>,
}

impl fmt::Display for DeviceInfo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name)
  }
}

impl DeviceInfo {
  pub fn available_devices() -> Option<Vec<DeviceInfo>> {
    let mut available_devices: Vec<DeviceInfo> = Vec::new();
    unsafe {
      let device_count = waveInGetNumDevs() as usize;
      if device_count == 0 {
        return None;
      }
      for device_index in 0..device_count {
        let size = size_of::<WAVEINCAPSW>() as u32;
        let mut device_capabilities = zeroed::<WAVEINCAPSW>();
        if waveInGetDevCapsW(device_index, &mut device_capabilities, size) != MMSYSERR_NOERROR {
          continue;
        }
        let name = match String::from_utf16(&device_capabilities.szPname) {
          Ok(res) => res,
          _ => continue,
        };
        let formats = DeviceFormat::unpack(device_capabilities.dwFormats);
        let info = DeviceInfo { index: device_index, name, formats };
        if info.formats.is_empty() {
          continue;
        }
        available_devices.push(info);
      }
    }
    Some(available_devices)
  }

  pub fn get_best_format(&self) -> DeviceFormat {
    let frequencies = DeviceFormat::relevant_frequencies();
    for &frequency in &frequencies {
      let mut freq_formats: Vec<&DeviceFormat> = self.formats.iter()
        .filter(|x| x.frequency == frequency)
        .collect();
      if !freq_formats.is_empty() {
        freq_formats.sort_by_key(|x| x.bits);
        freq_formats.sort_by_key(|x| x.channels);
        return freq_formats[0].clone();
      }
    }
    panic!("should not happen")
  }
}
