mod format;

use format::*;
use std::io::stdout;
use std::io::Write;
use std::{
  fmt,
  io::stdin,
  mem::{size_of, zeroed},
};
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
    // write!(f, "{}\n      [", self.name)?;
    // for (i, v) in self.formats.iter().enumerate() {
    //   if i != 0 {
    //     write!(f, ",\n       {}", v)?;
    //   } else {
    //     write!(f, "{}", v)?;
    //   }
    // }
    // write!(f, "]")
  }
}

impl DeviceInfo {
  pub fn from_selection_dialog() -> Option<DeviceInfo> {
    println!("select your audio device:");
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
        println!(" [{}] {}", device_index, info);
        available_devices.push(info);
      }
    }
    loop {
      print!("> ");
      stdout().flush().unwrap();
      let mut user_selected_device_string = String::new();
      if stdin().read_line(&mut user_selected_device_string).is_err() {
        println!("you should write number of your device!");
        continue;
      };
      let user_selected_device = match user_selected_device_string.trim().parse::<usize>() {
        Ok(res) => res,
        Err(_) => {
          println!("you should write number of your device!");
          continue;
        }
      };
      match available_devices.iter().find(|&x| x.index == user_selected_device) {
        None => println!("device #{} not in device list!", user_selected_device),
        Some(device) => return Some(device.clone()),
      };
    }
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
