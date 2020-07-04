use winapi::{shared::minwindef::DWORD, um::mmsystem::*};

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
    res.push("WHDR_DONE")
  };
  if whdr & WHDR_PREPARED != 0 {
    res.push("WHDR_PREPARED")
  };
  if whdr & WHDR_BEGINLOOP != 0 {
    res.push("WHDR_BEGINLOOP")
  };
  if whdr & WHDR_ENDLOOP != 0 {
    res.push("WHDR_ENDLOOP")
  };
  if whdr & WHDR_INQUEUE != 0 {
    res.push("WHDR_INQUEUE")
  };
  res.join(",")
}
