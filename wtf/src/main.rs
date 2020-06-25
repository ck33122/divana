// #![feature(const_fn)]

mod device_info;

use device_info::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

#[derive(Copy, Clone)]
enum Command {
  MainMenu,
  SetupDevice,
}

fn process_user_input() -> Option<String> {
  let mut user_input = String::new();
  print!("> ");
  stdout().flush().unwrap();
  if stdin().read_line(&mut user_input).is_err() {
    println!("input error!");
    return None
  };
  Some(user_input.trim().to_string())
}

// always constructed because of rust's lacks of static initialization 
fn command_map() -> HashMap<&'static str, Command> {
  [("setup device", Command::SetupDevice)]
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
        let cmd_name = match process_user_input() {
          Some(command_name) => command_name,
          None => continue
        };
        match command_map().get(cmd_name.as_str()) {
          Some(command) => current_command = (*command).clone(),
          None => show_help()
        };
      }
      Command::SetupDevice => {
        let device = DeviceInfo::from_selection_dialog();
        match device {
          Some(device) => {
            println!("selected device: {}", device);
            let fmt_info = device.get_best_format();
            println!("current format: {}", fmt_info.format);
          }
          None => println!("no device is currently selected (may be there is no devices in your computer?)"),
        }
        current_command = Command::MainMenu
      }
    }
  }
}
