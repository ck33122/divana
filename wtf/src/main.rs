mod device_info;

use device_info::*;
use std::collections::HashMap;

trait Command {
  fn run(&self);
}

struct SetupDevice {}

impl Command for SetupDevice {
  fn run(&self) {
    let device = DeviceInfo::from_selection_dialog();
    match device {
      Some(device) => {
        println!("selected device: {}", device);
        let fmt_info = device.get_best_format();
        println!("current format: {}", fmt_info.format);
      }
      None => println!("no device selected (may be there is no devices in your computer?)"),
    }
  }
}

fn main() {
  let cmd_setup_device = SetupDevice {};

  let mut commands: HashMap<&str, &dyn Command> = HashMap::new();
  commands.insert("setup device", &cmd_setup_device);

  // std::thread::spawn(move || loop {
  //   std::thread::sleep(std::time::Duration::from_millis(500));
  //   println!("[notification] sosi hui");
  // });
}
