mod ui;
mod device_info;

use device_info::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Command {
  MainMenu,
  SetupDevice,
  Exit,
}

// always constructed because of rust's lacks of static initialization 
fn command_map() -> HashMap<&'static str, Command> {
  [
    ("setup device", Command::SetupDevice),
    ("exit", Command::Exit)
  ]
    .iter().cloned().collect()
}

fn show_help() {
  println!("available commands:");
  for (command_name, _) in command_map() {
    println!("  {}", command_name);
  }
}

fn main() {
  let mut current_command = Command::MainMenu;
  loop {
    match current_command {
      Command::MainMenu => {
        let cmd_name = match ui::process_user_input() {
          Some(command_name) => command_name,
          None => continue
        };
        match command_map().get(cmd_name.as_str()) {
          Some(command) => current_command = (*command).clone(),
          None => show_help()
        };
      }
      Command::Exit => {
        return;
      }
      Command::SetupDevice => {
        let devices = match DeviceInfo::available_devices() {
          Some(devices) => devices,
          None => {
            current_command = Command::MainMenu;
            println!("no device is currently available (may be there is no devices in your computer?)");
            continue
          }
        };
        let device = match ui::process_select_one_of(devices) {
          Some(device) => device,
          None => {
            current_command = Command::MainMenu;
            println!("no device is currently selected (may be there is no devices in your computer?)");
            continue
          }
        };
        println!("selected device: {}", device);
        let format = device.get_best_format();
        println!("current format: {}", format);
        current_command = Command::MainMenu
      }
    }
  }
}
