#[macro_use(lazy_static)]
extern crate lazy_static;

mod device;
mod ui;

use device::{info::*, input::*, output::*};

#[derive(Copy, Clone)]
enum Command {
  MainMenu,
  SetupInput,
  Exit,
  Start,
  SetupOutput,
  Stop,
}

type CommandDefinition = (&'static str, Command);

struct DeviceSelection {
  device: DeviceInfo,
  format: DeviceFormat,
}
struct GlobalState {
  input_selection: Option<DeviceSelection>,
  output_selection: Option<DeviceSelection>,
  input: Option<InputDevice>,
  output: Option<OutputDevice>,
}

lazy_static! {
  static ref COMMAND_MAP: [CommandDefinition; 5] = [
    ("input", Command::SetupInput),
    ("output", Command::SetupOutput),
    ("exit", Command::Exit),
    ("start", Command::Start),
    ("stop", Command::Stop),
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
    input_selection: None,
    output_selection: None,
    input: None,
    output: None,
  };
  //-------------------------------------------------------------------- DEBUG STUFF
  let (input_devices, output_devices) = (DeviceInfo::input_devices(), DeviceInfo::output_devices());
  let (input_device, output_device) = (input_devices.first().unwrap().clone(), output_devices.first().unwrap().clone());
  state.input_selection = Some(DeviceSelection {
    device: input_device.clone(),
    format: input_device.get_best_format(),
  });
  state.output_selection = Some(DeviceSelection {
    device: output_device.clone(),
    format: output_device.get_best_format(),
  });
  //-------------------------------------------------------------------- DEBUG STUFF
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
        state.input_selection = None;
        state.output_selection = None;
        state.input = None;
        return;
      }
      Command::SetupInput => {
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
        state.input_selection = Some(DeviceSelection { device, format });
      }
      Command::SetupOutput => {
        current_command = Command::MainMenu;
        // if state.input.is_some() {
        //   something_is_wrong();
        //   println!("cannot setup device because some device already used. need to stop it first");
        //   continue;
        // }
        let device = match ui::process_select_one_of(DeviceInfo::output_devices()) {
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
        state.output_selection = Some(DeviceSelection { device, format });
      }
      Command::Start => {
        current_command = Command::MainMenu;
        if state.input.as_ref().is_some() {
          something_is_wrong();
          println!("could not start input because it is already started");
          continue;
        }
        if state.output.as_ref().is_some() {
          something_is_wrong();
          println!("could not start output because it is already started");
          continue;
        }
        let in_selection = match &state.input_selection {
          Some(v) => v,
          None => {
            something_is_wrong();
            println!("no input device selected (before starting select device using \"input\" command)");
            continue;
          }
        };
        let out_selection = match &state.output_selection {
          Some(v) => v,
          None => {
            something_is_wrong();
            println!("no output device selected (before starting select device using \"output\" command)");
            continue;
          }
        };
        println!(
          "trying to open output for {} with format {}",
          out_selection.device, out_selection.format
        );
        state.output = Some(OutputDevice::new(out_selection.format, out_selection.device.index));
        println!(
          "trying to open input for {} with format {}",
          in_selection.device, in_selection.format
        );
        state.input = Some(InputDevice::new(
          in_selection.format,
          in_selection.device.index,
          state.output.as_ref().unwrap().sender.clone(),
        ));
      }
      Command::Stop => {
        current_command = Command::MainMenu;
        state.input = None;
        state.output = None;
      }
    }
  }
}
