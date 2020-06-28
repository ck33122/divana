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
  StopInput,
}

type CommandDefinition = (&'static str, Command);

struct DeviceSelection {
  device: DeviceInfo,
  format: DeviceFormat,
}
struct GlobalState {
  device_selection: Option<DeviceSelection>,
  input: Option<InputDevicePtr>,
  sender: Option<SenderThread>,
}

lazy_static! {
  static ref COMMAND_MAP: [CommandDefinition; 4] = [
    ("device", Command::SetupDevice),
    ("exit", Command::Exit),
    ("input", Command::StartInput),
    ("stop", Command::StopInput),
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
    input: None,
    sender: None,
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
        state.input = None;
        state.sender.take().map(|ref mut t| t.stop());
        return;
      }
      Command::SetupDevice => {
        current_command = Command::MainMenu;
        if state.input.is_some() {
          something_is_wrong();
          println!("cannot setup device because some device already used. need to stop it first");
          continue;
        }
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
        if state.input.is_some() {
          something_is_wrong();
          println!("could not start input because it is already started");
          continue;
        }
        let selection = match &state.device_selection {
          Some(v) => v,
          None => {
            something_is_wrong();
            println!("no device selected (before starting input select device using \"device\" command)");
            continue;
          }
        };
        state.sender.take().map(|ref mut t| t.stop());
        state.sender = Some(SenderThread::new());
        println!("trying to open input for {} with format {}", selection.device, selection.format);
        match DeviceInfo::open_input_stream(selection.format, selection.device.index, state.sender.as_ref().unwrap()) {
          Err(err) => {
            something_is_wrong();
            println!("open input stream error: {}", err);
            continue;
          }
          Ok(input) => state.input = Some(input),
        }
        println!("input opened!");
      }
      Command::StopInput => {
        current_command = Command::MainMenu;
        state.input = None;
        state.sender.take().map(|ref mut t| t.stop());
        println!("stop done!");
      }
    }
  }
}
