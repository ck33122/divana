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
    ("device", Command::SetupDevice),
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
        let device = match ui::process_select_one_of(DeviceInfo::input_devices()) {
          Some(device) => device,
          None => {
            println!("no device is currently available (may be there is no devices in your computer?)");
            current_command = Command::MainMenu;
            continue
          }
        };
        let format = device.get_best_format();
        println!("selected device: {}", device);
        println!("current format: {}", format);
        current_command = Command::MainMenu
      }
    }
  }
}
