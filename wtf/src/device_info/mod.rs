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
      let devnum = waveInGetNumDevs() as usize;
      if devnum == 0 {
        return None;
      }
      for index in 0..devnum {
        let size = size_of::<WAVEINCAPSW>() as u32;
        let mut devcaps = zeroed::<WAVEINCAPSW>();
        if waveInGetDevCapsW(index, &mut devcaps, size) != MMSYSERR_NOERROR {
          continue;
        }
        let name = match String::from_utf16(&devcaps.szPname) {
          Ok(res) => res,
          _ => continue,
        };
        let formats = DeviceFormat::unpack(devcaps.dwFormats);
        let info = DeviceInfo { index, name, formats };
        if info.formats.is_empty() {
          continue;
        }
        println!(" [{}] {}", index, info);
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

  pub fn get_best_format(&self) -> DeviceFormatInfo {
    // let formats: Vec<DeviceFormatInfo> = self.formats.iter().map(|&x| x.get_info()).collect();
    let freqs: [usize; 4] = [44100, 96000, 22050, 11025];
    for &freq in &freqs {
      let mut freq_formats: Vec<DeviceFormatInfo> = self.formats.iter().map(|&x| x.get_info()).filter(|x| x.frequency == freq).collect();
      if !freq_formats.is_empty() {
        freq_formats.sort_by_key(|x| x.bits);
        freq_formats.sort_by_key(|x| x.channels);
        return freq_formats[0];
      }
    }
    panic!("should not happen")
  }
}
