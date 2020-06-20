use std::{io::*, mem::*, string::*};
use winapi::um::{mmeapi::*, mmsystem::*};

fn main() {
  let device = dev_selection_dialog();
  println!("selected device: {}", device)
}

fn dev_selection_dialog() -> usize {
  println!("select your audio device:");
  let mut available_devices: Vec<usize> = Vec::new();
  unsafe {
    let devnum = waveInGetNumDevs() as usize;
    if devnum == 0 {
      panic!("NO AVAILABLE DEVICES")
    }
    for devindex in 0..devnum {
      let size = size_of::<WAVEINCAPSW>() as u32;
      let mut devcaps = zeroed::<WAVEINCAPSW>();
      if waveInGetDevCapsW(devindex, &mut devcaps, size) != MMSYSERR_NOERROR {
        continue;
      }
      let name = match String::from_utf16(&devcaps.szPname) {
        Ok(res) => res,
        _ => continue,
      };
      available_devices.push(devindex);
      println!(" [{}] {}", devindex, name);
    }
  }

  loop {
    print!("> ");
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
    if available_devices
      .iter()
      .find(|&&x| x == user_selected_device)
      .is_none()
    {
      println!("device #{} not in device list!", user_selected_device)
    } else {
      return user_selected_device;
    }
  }
}
