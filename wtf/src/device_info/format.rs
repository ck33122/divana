use std::fmt;
use winapi::{shared::minwindef::DWORD, um::mmsystem::*};

// on_device_format: ($const_name: ident, $const_dword: expr, $frequency: expr, $channels: expr, $bits: expr)
macro_rules! enumerate_device_formats {
  ($on_device_format: ident) => {
    $on_device_format!(F11025hzMono8bit, WAVE_FORMAT_1M08, 11025usize, 1usize, 8usize)
    $on_device_format!(F11025hzMono16bit, WAVE_FORMAT_1M16, 11025usize, 1usize, 16usize)
    $on_device_format!(F11025hzStereo8bit, WAVE_FORMAT_1S08, 11025usize, 2usize, 8usize)
    $on_device_format!(F11025hzStereo16bit, WAVE_FORMAT_1S16, 11025usize, 2usize, 16usize)
    $on_device_format!(F22050hzMono8bit, WAVE_FORMAT_2M08, 22050usize, 1usize, 8usize)
    $on_device_format!(F22050hzMono16bit, WAVE_FORMAT_2M16, 22050usize, 1usize, 16usize)
    $on_device_format!(F22050hzStereo8bit, WAVE_FORMAT_2S08, 22050usize, 2usize, 8usize)
    $on_device_format!(F22050hzStereo16bit, WAVE_FORMAT_2S16, 22050usize, 2usize, 16usize)
    $on_device_format!(F44100hzMono8bit, WAVE_FORMAT_4M08, 44100usize, 1usize, 8usize)
    $on_device_format!(F44100hzMono16bit, WAVE_FORMAT_4M16, 44100usize, 1usize, 16usize)
    $on_device_format!(F44100hzStereo8bit, WAVE_FORMAT_4S08, 44100usize, 2usize, 8usize)
    $on_device_format!(F44100hzStereo16bit, WAVE_FORMAT_4S16, 44100usize, 2usize, 16usize)
    $on_device_format!(F96000hzMono8bit, WAVE_FORMAT_96M08, 96000usize, 1usize, 8usize)
    $on_device_format!(F96000hzMono16bit, WAVE_FORMAT_96M16, 96000usize, 1usize, 16usize)
    $on_device_format!(F96000hzStereo8bit, WAVE_FORMAT_96S08, 96000usize, 2usize, 8usize)
    $on_device_format!(F96000hzStereo16bit, WAVE_FORMAT_96S16, 96000usize, 2usize, 16usize)
  }
}

#[derive(Clone, Copy)]
pub struct DeviceFormat {
  pub format: DWORD,
  pub frequency: usize,
  pub channels: usize,
  pub bits: usize,
}

impl DeviceFormat {
  pub fn unpack(packed_format: DWORD) -> Vec<DeviceFormat> {
    let mut result: Vec<DeviceFormat> = Vec::new();
    macro_rules! expand_device_format_enum {
      ($const_name: ident, $const_dword: expr, $frequency: expr, $channels: expr, $bits: expr) => {
        if packed_format & $const_dword != 0 {
          result.push(DeviceFormat {
            format: $const_dword,
            frequency: $frequency,
            channels: $channels,
            bits: $bits,
          })
        }
      }
    }
    enumerate_device_formats!(expand_device_format_enum);
    result
  }
  pub fn relevant_frequencies() -> [usize; 4] {
    [44100, 96000, 22050, 11025]
  }
}

impl fmt::Display for DeviceFormat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}hz {} {}bit",
      self.frequency,
      if self.channels == 1 { "Mono" } else { "Stereo" },
      self.bits
    )
  }
}
