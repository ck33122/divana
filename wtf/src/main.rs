#[macro_use(lazy_static)]
extern crate lazy_static;

mod device_info;
mod ui;

use device_info::*;

#[derive(Copy, Clone)]
enum Command {
  MainMenu,
  SetupDevice,
  Exit,
  StartInput,
}

type CommandDefinition = (&'static str, Command);

struct DeviceSelection {
  device: DeviceInfo,
  format: DeviceFormat,
}
struct GlobalState {
  device_selection: Option<DeviceSelection>,
  input: *mut InputDevice,
}

lazy_static! {
  static ref COMMAND_MAP: [CommandDefinition; 3] = [
    ("device", Command::SetupDevice),
    ("exit", Command::Exit),
    ("input", Command::StartInput)
  ];
}

fn show_help() {
  println!("available commands:");
  for (command_name, _) in COMMAND_MAP.iter() {
    println!("  {}", command_name);
  }
}

fn something_is_wrong() {
  println!("\\_(@u@)_/");
  // use winapi::um::winuser::{MessageBeep, MB_ICONERROR};
  // unsafe { MessageBeep(MB_ICONERROR) };
}

fn main() {
  let mut state = GlobalState {
    device_selection: None,
    input: InputDevice::null(),
  };
  let mut current_command = Command::MainMenu;
  loop {
    match current_command {
      Command::MainMenu => {
        let cmd_name = match ui::process_user_input() {
          Some(command_name) => command_name,
          None => {
            something_is_wrong();
            continue;
          }
        };
        let matched_commands: Vec<CommandDefinition> = COMMAND_MAP
          .iter()
          .filter(|(k, _)| k.starts_with(cmd_name.as_str()))
          .cloned()
          .collect();
        match matched_commands.len() {
          1 => current_command = matched_commands[0].1.clone(),
          0 => {
            println!("there is no commands matched to your query");
            show_help();
            something_is_wrong();
          }
          _ => {
            println!("there is multiple commands matched to your query:");
            for (key, _) in matched_commands.iter() {
              println!("  {}", key);
            }
            println!("please, be more explicit");
            something_is_wrong();
          }
        };
      }
      Command::Exit => {
        state.device_selection = None;
        InputDevice::free(state.input);
        return;
      }
      Command::SetupDevice => {
        current_command = Command::MainMenu;
        let device = match ui::process_select_one_of(DeviceInfo::input_devices()) {
          Some(device) => device,
          None => {
            something_is_wrong();
            println!("no device is currently available (may be there is no devices in your computer?)");
            continue;
          }
        };
        let format = device.get_best_format();
        println!("selected device: {}", device);
        println!("current format: {}", format);
        state.device_selection = Some(DeviceSelection { device, format });
      }
      Command::StartInput => {
        current_command = Command::MainMenu;
        let selection = match &state.device_selection {
          Some(v) => v,
          None => {
            something_is_wrong();
            println!("no device selected (before starting input select device using \"device\" command)");
            continue;
          }
        };
        println!("trying to open input for {} with format {}", selection.device, selection.format);
        match DeviceInfo::open_input_stream(selection.format, selection.device.index) {
          Err(err) => {
            something_is_wrong();
            println!("open input stream error: {}", err);
            continue;
          }
          Ok(input) => state.input = input,
        }
        println!("input opened!");
      }
    }
  }
}
