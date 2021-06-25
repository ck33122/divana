#[macro_use(lazy_static)]
extern crate lazy_static;

mod device;
mod ui;
mod vorbis;

use device::{info::*, input::*, output::*};
use portaudio as pa;
use vorbis::ogg;

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

fn pa_test() -> std::result::Result<(), pa::Error> {
  const SAMPLE_RATE: f64 = 44_100.0;
  const FRAMES: u32 = 256;
  const CHANNELS: i32 = 2;
  const INTERLEAVED: bool = true;

  let pa = pa::PortAudio::new()?;
  let default_host = pa.default_host_api()?;
  println!("default host: {:#?}", pa.host_api_info(default_host));
  let def_input = pa.default_input_device()?;
  let input_info = pa.device_info(def_input)?;
  println!("Default input device info: {:#?}", &input_info);
  let latency = input_info.default_low_input_latency;
  let input_params = pa::StreamParameters::<f32>::new(def_input, CHANNELS, INTERLEAVED, latency);
  let def_output = pa.default_output_device()?;
  let output_info = pa.device_info(def_output)?;
  println!("Default output device info: {:#?}", &output_info);
  let latency = output_info.default_low_output_latency;
  let output_params = pa::StreamParameters::new(def_output, CHANNELS, INTERLEAVED, latency);
  pa.is_duplex_format_supported(input_params, output_params, SAMPLE_RATE)?;
  let settings = pa::DuplexStreamSettings::new(input_params, output_params, SAMPLE_RATE, FRAMES);

  let mut count_down = 20.0;
  let mut maybe_last_time = None;
  let (sender, receiver) = ::std::sync::mpsc::channel();
  // A callback to pass to the non-blocking stream.
  let callback = move |pa::DuplexStreamCallbackArgs {
                         in_buffer,
                         out_buffer,
                         frames,
                         time,
                         ..
                       }| {
    let current_time = time.current;
    let prev_time = maybe_last_time.unwrap_or(current_time);
    let dt = current_time - prev_time;
    count_down -= dt;
    maybe_last_time = Some(current_time);

    assert!(frames == FRAMES as usize);
    sender.send(count_down).ok();

    // println!("{:#?}", out_buffer);

    for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
      *output_sample = *input_sample;
    }

    if count_down > 0.0 {
      pa::Continue
    } else {
      pa::Complete
    }
  };

  let mut stream = pa.open_non_blocking_stream(settings, callback)?;
  stream.start()?;

  // Loop while the non-blocking stream is active.
  while let true = stream.is_active()? {
    // Do some stuff!
    while let Ok(count_down) = receiver.try_recv() {
      // println!("count_down: {:?}", count_down);
    }
  }

  stream.stop()?;
  Ok(())
}

fn main() {
  match pa_test() {
    Ok(_) => {}
    e => {
      eprintln!("Example failed with the following: {:?}", e);
    }
  }
  println!("exiting!");
  return;
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
