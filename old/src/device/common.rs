use {
  std::{
    alloc::{alloc_zeroed, dealloc, Layout},
    ptr::copy_nonoverlapping,
  },
  winapi::{shared::minwindef::DWORD, um::mmsystem::*},
};

pub struct WaveBuffer {
  pub data: *mut i8,
  layout: Layout,
  size: usize,
}

unsafe impl Send for WaveBuffer {}

impl WaveBuffer {
  pub fn new(size: usize) -> Self {
    let layout = match Layout::array::<i8>(size) {
      Ok(layout) => layout,
      Err(err) => panic!("cannot determine buffer layout: {}", err),
    };
    let data = unsafe { alloc_zeroed(layout) as *mut i8 };
    Self { data, layout, size }
  }

  pub fn length(&self) -> u32 {
    self.size as u32
  }

  pub fn copy_to(&self, dst: &mut Self) {
    if self.size > dst.size {
      panic!("original buffer size is bigger than destination, cannot perform copy_to")
    }
    unsafe { copy_nonoverlapping(self.data, dst.data, self.size) };
  }

  pub fn partially_clone(&self, partition_size: u32) -> Self {
    if partition_size > self.size as u32 {
      panic!("partition_size could not be bigger than original size")
    }
    let buffer = WaveBuffer::new(partition_size as usize);
    unsafe { copy_nonoverlapping(self.data, buffer.data, partition_size as usize) };
    buffer
  }
}

impl Drop for WaveBuffer {
  fn drop(&mut self) {
    println!("WAVEBUFFER DROP");
    unsafe { dealloc(self.data as *mut u8, self.layout) }
  }
}

pub fn mm_error_to_string(r: MMRESULT) -> &'static str {
  match r {
    MMSYSERR_NOERROR => "NOERROR",
    MMSYSERR_ERROR => "ERROR",
    MMSYSERR_BADDEVICEID => "BADDEVICEID",
    MMSYSERR_NOTENABLED => "NOTENABLED",
    MMSYSERR_ALLOCATED => "ALLOCATED",
    MMSYSERR_INVALHANDLE => "INVALHANDLE",
    MMSYSERR_NODRIVER => "NODRIVER",
    MMSYSERR_NOMEM => "NOMEM",
    MMSYSERR_NOTSUPPORTED => "NOTSUPPORTED",
    MMSYSERR_BADERRNUM => "BADERRNUM",
    MMSYSERR_INVALFLAG => "INVALFLAG",
    MMSYSERR_INVALPARAM => "INVALPARAM",
    MMSYSERR_HANDLEBUSY => "HANDLEBUSY",
    MMSYSERR_INVALIDALIAS => "INVALIDALIAS",
    MMSYSERR_BADDB => "BADDB",
    MMSYSERR_KEYNOTFOUND => "KEYNOTFOUND",
    MMSYSERR_READERROR => "READERROR",
    MMSYSERR_WRITEERROR => "WRITEERROR",
    MMSYSERR_DELETEERROR => "DELETEERROR",
    MMSYSERR_VALNOTFOUND => "VALNOTFOUND",
    MMSYSERR_NODRIVERCB => "NODRIVERCB",
    MMSYSERR_MOREDATA => "MOREDATA",
    WAVERR_BADFORMAT => "BADFORMAT",
    WAVERR_STILLPLAYING => "STILLPLAYING",
    WAVERR_UNPREPARED => "UNPREPARED",
    WAVERR_SYNC => "SYNC",
    _ => "[unknown (not MM-system error)]",
  }
}

pub const WHDR_DONE: DWORD = 0x00000001; // done bit
pub const WHDR_PREPARED: DWORD = 0x00000002; // set if this header has been prepared
pub const WHDR_BEGINLOOP: DWORD = 0x00000004; // loop start block
pub const WHDR_ENDLOOP: DWORD = 0x00000008; // loop end block
pub const WHDR_INQUEUE: DWORD = 0x00000010; // reserved for driver

pub fn whdr_to_str(whdr: DWORD) -> String {
  let mut res = vec![];
  if whdr & WHDR_DONE != 0 {
    res.push("WHDR_DONE".into())
  };
  if whdr & WHDR_PREPARED != 0 {
    res.push("WHDR_PREPARED".into())
  };
  if whdr & WHDR_BEGINLOOP != 0 {
    res.push("WHDR_BEGINLOOP".into())
  };
  if whdr & WHDR_ENDLOOP != 0 {
    res.push("WHDR_ENDLOOP".into())
  };
  if whdr & WHDR_INQUEUE != 0 {
    res.push("WHDR_INQUEUE".into())
  };
  if res.len() == 0 {
    res.push(format!("UNKNOWN {}", whdr))
  }
  res.join(",")
}
