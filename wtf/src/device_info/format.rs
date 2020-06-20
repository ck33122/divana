use std::fmt;
use winapi::{shared::minwindef::DWORD, um::mmsystem::*};

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum DeviceFormat {
  F11025hzMono8bit = WAVE_FORMAT_1M08,
  F11025hzMono16bit = WAVE_FORMAT_1M16,
  F11025hzStereo8bit = WAVE_FORMAT_1S08,
  F11025hzStereo16bit = WAVE_FORMAT_1S16,
  F22050hzMono8bit = WAVE_FORMAT_2M08,
  F22050hzMono16bit = WAVE_FORMAT_2M16,
  F22050hzStereo8bit = WAVE_FORMAT_2S08,
  F22050hzStereo16bit = WAVE_FORMAT_2S16,
  F44100hzMono8bit = WAVE_FORMAT_4M08,
  F44100hzMono16bit = WAVE_FORMAT_4M16,
  F44100hzStereo8bit = WAVE_FORMAT_4S08,
  F44100hzStereo16bit = WAVE_FORMAT_4S16,
  F96000hzMono8bit = WAVE_FORMAT_96M08,
  F96000hzMono16bit = WAVE_FORMAT_96M16,
  F96000hzStereo8bit = WAVE_FORMAT_96S08,
  F96000hzStereo16bit = WAVE_FORMAT_96S16,
}

#[derive(Clone, Copy)]
pub struct DeviceFormatInfo {
  pub format: DeviceFormat,
  pub frequency: usize,
  pub channels: usize,
  pub bits: usize,
}

impl DeviceFormat {
  pub fn as_dword(self) -> DWORD {
    self as DWORD
  }
  pub fn unpack(packed_format: DWORD) -> Vec<DeviceFormat> {
    let mut result: Vec<DeviceFormat> = Vec::new();
    let formats = [
      DeviceFormat::F11025hzMono8bit,
      DeviceFormat::F11025hzMono16bit,
      DeviceFormat::F11025hzStereo8bit,
      DeviceFormat::F11025hzStereo16bit,
      DeviceFormat::F22050hzMono8bit,
      DeviceFormat::F22050hzMono16bit,
      DeviceFormat::F22050hzStereo8bit,
      DeviceFormat::F22050hzStereo16bit,
      DeviceFormat::F44100hzMono8bit,
      DeviceFormat::F44100hzMono16bit,
      DeviceFormat::F44100hzStereo8bit,
      DeviceFormat::F44100hzStereo16bit,
      DeviceFormat::F96000hzMono8bit,
      DeviceFormat::F96000hzMono16bit,
      DeviceFormat::F96000hzStereo8bit,
      DeviceFormat::F96000hzStereo16bit,
    ];
    for &format in &formats {
      if packed_format & format.as_dword() != 0 {
        result.push(format);
      }
    }
    result
  }
  pub fn get_info(&self) -> DeviceFormatInfo {
    match self {
      DeviceFormat::F11025hzMono8bit => DeviceFormatInfo {
        format: DeviceFormat::F11025hzMono8bit,
        frequency: 11025,
        channels: 1,
        bits: 8,
      },
      DeviceFormat::F11025hzMono16bit => DeviceFormatInfo {
        format: DeviceFormat::F11025hzMono16bit,
        frequency: 11025,
        channels: 1,
        bits: 16,
      },
      DeviceFormat::F11025hzStereo8bit => DeviceFormatInfo {
        format: DeviceFormat::F11025hzStereo8bit,
        frequency: 11025,
        channels: 2,
        bits: 8,
      },
      DeviceFormat::F11025hzStereo16bit => DeviceFormatInfo {
        format: DeviceFormat::F11025hzStereo16bit,
        frequency: 11025,
        channels: 2,
        bits: 16,
      },
      DeviceFormat::F22050hzMono8bit => DeviceFormatInfo {
        format: DeviceFormat::F22050hzMono8bit,
        frequency: 22050,
        channels: 1,
        bits: 8,
      },
      DeviceFormat::F22050hzMono16bit => DeviceFormatInfo {
        format: DeviceFormat::F22050hzMono16bit,
        frequency: 22050,
        channels: 1,
        bits: 16,
      },
      DeviceFormat::F22050hzStereo8bit => DeviceFormatInfo {
        format: DeviceFormat::F22050hzStereo8bit,
        frequency: 22050,
        channels: 2,
        bits: 8,
      },
      DeviceFormat::F22050hzStereo16bit => DeviceFormatInfo {
        format: DeviceFormat::F22050hzStereo16bit,
        frequency: 22050,
        channels: 2,
        bits: 16,
      },
      DeviceFormat::F44100hzMono8bit => DeviceFormatInfo {
        format: DeviceFormat::F44100hzMono8bit,
        frequency: 44100,
        channels: 1,
        bits: 8,
      },
      DeviceFormat::F44100hzMono16bit => DeviceFormatInfo {
        format: DeviceFormat::F44100hzMono16bit,
        frequency: 44100,
        channels: 1,
        bits: 16,
      },
      DeviceFormat::F44100hzStereo8bit => DeviceFormatInfo {
        format: DeviceFormat::F44100hzStereo8bit,
        frequency: 44100,
        channels: 2,
        bits: 8,
      },
      DeviceFormat::F44100hzStereo16bit => DeviceFormatInfo {
        format: DeviceFormat::F44100hzStereo16bit,
        frequency: 44100,
        channels: 2,
        bits: 16,
      },
      DeviceFormat::F96000hzMono8bit => DeviceFormatInfo {
        format: DeviceFormat::F96000hzMono8bit,
        frequency: 96000,
        channels: 1,
        bits: 8,
      },
      DeviceFormat::F96000hzMono16bit => DeviceFormatInfo {
        format: DeviceFormat::F96000hzMono16bit,
        frequency: 96000,
        channels: 1,
        bits: 16,
      },
      DeviceFormat::F96000hzStereo8bit => DeviceFormatInfo {
        format: DeviceFormat::F96000hzStereo8bit,
        frequency: 96000,
        channels: 2,
        bits: 8,
      },
      DeviceFormat::F96000hzStereo16bit => DeviceFormatInfo {
        format: DeviceFormat::F96000hzStereo16bit,
        frequency: 96000,
        channels: 2,
        bits: 16,
      },
    }
  }
}

impl fmt::Display for DeviceFormat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let info = self.get_info();
    write!(
      f,
      "{}hz {} {}bit",
      info.frequency,
      if info.channels == 1 { "Mono" } else { "Stereo" },
      info.bits
    )
  }
}
