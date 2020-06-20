mod device_info;

use device_info::*;

fn main() {
  let device = DeviceInfo::from_selection_dialog();
  match device {
    Some(device) => println!("selected device: {}", device),
    None => println!("no device selected"),
  }
}
