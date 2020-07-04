#[allow(dead_code, non_camel_case_types, unused_imports)]
pub mod ogg {
  use std::os::raw::*;

  pub type size_t = c_ulonglong;
  pub type ogg_int16_t = i16;
  pub type ogg_uint16_t = u16;
  pub type ogg_int32_t = i32;
  pub type ogg_uint32_t = u32;
  pub type ogg_int64_t = i64;
  pub type ogg_uint64_t = u64;

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ogg_iovec_t {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
  }

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ogg_sync_state {
    pub data: *mut c_uchar,
    pub storage: c_int,
    pub fill: c_int,
    pub returned: c_int,
    pub unsynced: c_int,
    pub headerbytes: c_int,
    pub bodybytes: c_int,
  }

  // ogg_packet is used to encapsulate the data and metadata belonging to a single raw Ogg/Vorbis packet
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ogg_packet {
    pub packet: *mut c_uchar,
    pub bytes: c_long,
    pub b_o_s: c_long,
    pub e_o_s: c_long,
    pub granulepos: ogg_int64_t,
    pub packetno: ogg_int64_t, /* sequence number for decode; the framing knows where there's a hole in the data,
                               but we need coupling so that the codec (which is in a separate abstractionlayer)
                               also knows about the gap */
  }

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct oggpack_buffer {
    pub endbyte: c_long,
    pub endbit: c_int,
    pub buffer: *mut c_uchar,
    pub ptr: *mut c_uchar,
    pub storage: c_long,
  }

  // ogg_page is used to encapsulate the data in one Ogg bitstream page
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ogg_page {
    pub header: *mut c_uchar,
    pub header_len: c_long,
    pub body: *mut c_uchar,
    pub body_len: c_long,
  }

  // ogg_stream_state contains the current encode/decode state of a logical Ogg bitstream
  #[repr(C)]
  #[derive(Copy, Clone)]
  pub struct ogg_stream_state {
    pub body_data: *mut c_uchar,        // bytes from packet bodies
    pub body_storage: c_long,           // storage elements allocated
    pub body_fill: c_long,              // elements stored; fill mark
    pub body_returned: c_long,          // elements of fill returned
    pub lacing_vals: *mut c_int,        // The values that will go to the segment table
    pub granule_vals: *mut ogg_int64_t, // granulepos values for headers. Not compact this way, but it is simple coupled to the lacing fifo
    pub lacing_storage: c_long,
    pub lacing_fill: c_long,
    pub lacing_packet: c_long,
    pub lacing_returned: c_long,
    pub header: [c_uchar; 282usize], // working space for header encode
    pub header_fill: c_int,
    pub e_o_s: c_int, // set when we have buffered the last packet in the logical bitstream
    pub b_o_s: c_int, // set after we've written the initial page of a logical bitstream
    pub serialno: c_long,
    pub pageno: c_long, /* sequence number for decode; the framing knows where there's a hole in the data,
                        but we need coupling so that the codec (which is in a separate abstractionlayer)
                        also knows about the gap */
    pub packetno: ogg_int64_t,
    pub granulepos: ogg_int64_t,
  }
  #[link(name = "libogg", kind = "static")]
  extern "C" {
    //
    // Ogg BITSTREAM PRIMITIVES: bitstream
    //
    pub fn oggpack_writeinit(b: *mut oggpack_buffer);
    pub fn oggpack_writecheck(b: *mut oggpack_buffer) -> c_int;
    pub fn oggpack_writetrunc(b: *mut oggpack_buffer, bits: c_long);
    pub fn oggpack_writealign(b: *mut oggpack_buffer);
    pub fn oggpack_writecopy(b: *mut oggpack_buffer, source: *mut c_void, bits: c_long);
    pub fn oggpack_reset(b: *mut oggpack_buffer);
    pub fn oggpack_writeclear(b: *mut oggpack_buffer);
    pub fn oggpack_readinit(b: *mut oggpack_buffer, buf: *mut c_uchar, bytes: c_int);
    pub fn oggpack_write(b: *mut oggpack_buffer, value: c_ulong, bits: c_int);
    pub fn oggpack_look(b: *mut oggpack_buffer, bits: c_int) -> c_long;
    pub fn oggpack_look1(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpack_adv(b: *mut oggpack_buffer, bits: c_int);
    pub fn oggpack_adv1(b: *mut oggpack_buffer);
    pub fn oggpack_read(b: *mut oggpack_buffer, bits: c_int) -> c_long;
    pub fn oggpack_read1(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpack_bytes(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpack_bits(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpack_get_buffer(b: *mut oggpack_buffer) -> *mut c_uchar;

    pub fn oggpackB_writeinit(b: *mut oggpack_buffer);
    pub fn oggpackB_writecheck(b: *mut oggpack_buffer) -> c_int;
    pub fn oggpackB_writetrunc(b: *mut oggpack_buffer, bits: c_long);
    pub fn oggpackB_writealign(b: *mut oggpack_buffer);
    pub fn oggpackB_writecopy(b: *mut oggpack_buffer, source: *mut c_void, bits: c_long);
    pub fn oggpackB_reset(b: *mut oggpack_buffer);
    pub fn oggpackB_writeclear(b: *mut oggpack_buffer);
    pub fn oggpackB_readinit(b: *mut oggpack_buffer, buf: *mut c_uchar, bytes: c_int);
    pub fn oggpackB_write(b: *mut oggpack_buffer, value: c_ulong, bits: c_int);
    pub fn oggpackB_look(b: *mut oggpack_buffer, bits: c_int) -> c_long;
    pub fn oggpackB_look1(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpackB_adv(b: *mut oggpack_buffer, bits: c_int);
    pub fn oggpackB_adv1(b: *mut oggpack_buffer);
    pub fn oggpackB_read(b: *mut oggpack_buffer, bits: c_int) -> c_long;
    pub fn oggpackB_read1(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpackB_bytes(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpackB_bits(b: *mut oggpack_buffer) -> c_long;
    pub fn oggpackB_get_buffer(b: *mut oggpack_buffer) -> *mut c_uchar;

    // Ogg BITSTREAM PRIMITIVES: encoding

    pub fn ogg_stream_packetin(os: *mut ogg_stream_state, op: *mut ogg_packet) -> c_int;
    pub fn ogg_stream_iovecin(
      os: *mut ogg_stream_state,
      iov: *mut ogg_iovec_t,
      count: c_int,
      e_o_s: c_long,
      granulepos: ogg_int64_t,
    ) -> c_int;
    pub fn ogg_stream_pageout(os: *mut ogg_stream_state, og: *mut ogg_page) -> c_int;
    pub fn ogg_stream_pageout_fill(os: *mut ogg_stream_state, og: *mut ogg_page, nfill: c_int) -> c_int;
    pub fn ogg_stream_flush(os: *mut ogg_stream_state, og: *mut ogg_page) -> c_int;
    pub fn ogg_stream_flush_fill(os: *mut ogg_stream_state, og: *mut ogg_page, nfill: c_int) -> c_int;

    // Ogg BITSTREAM PRIMITIVES: decoding

    pub fn ogg_sync_init(oy: *mut ogg_sync_state) -> c_int;
    pub fn ogg_sync_clear(oy: *mut ogg_sync_state) -> c_int;
    pub fn ogg_sync_reset(oy: *mut ogg_sync_state) -> c_int;
    pub fn ogg_sync_destroy(oy: *mut ogg_sync_state) -> c_int;
    pub fn ogg_sync_check(oy: *mut ogg_sync_state) -> c_int;

    pub fn ogg_sync_buffer(oy: *mut ogg_sync_state, size: c_long) -> *mut c_char;
    pub fn ogg_sync_wrote(oy: *mut ogg_sync_state, bytes: c_long) -> c_int;
    pub fn ogg_sync_pageseek(oy: *mut ogg_sync_state, og: *mut ogg_page) -> c_long;
    pub fn ogg_sync_pageout(oy: *mut ogg_sync_state, og: *mut ogg_page) -> c_int;
    pub fn ogg_stream_pagein(os: *mut ogg_stream_state, og: *mut ogg_page) -> c_int;
    pub fn ogg_stream_packetout(os: *mut ogg_stream_state, op: *mut ogg_packet) -> c_int;
    pub fn ogg_stream_packetpeek(os: *mut ogg_stream_state, op: *mut ogg_packet) -> c_int;

    // Ogg BITSTREAM PRIMITIVES: general

    pub fn ogg_stream_init(os: *mut ogg_stream_state, serialno: c_int) -> c_int;
    pub fn ogg_stream_clear(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_reset(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_reset_serialno(os: *mut ogg_stream_state, serialno: c_int) -> c_int;
    pub fn ogg_stream_destroy(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_check(os: *mut ogg_stream_state) -> c_int;
    pub fn ogg_stream_eos(os: *mut ogg_stream_state) -> c_int;

    pub fn ogg_page_checksum_set(og: *mut ogg_page);

    pub fn ogg_page_version(og: *const ogg_page) -> c_int;
    pub fn ogg_page_continued(og: *const ogg_page) -> c_int;
    pub fn ogg_page_bos(og: *const ogg_page) -> c_int;
    pub fn ogg_page_eos(og: *const ogg_page) -> c_int;
    pub fn ogg_page_granulepos(og: *const ogg_page) -> ogg_int64_t;
    pub fn ogg_page_serialno(og: *const ogg_page) -> c_int;
    pub fn ogg_page_pageno(og: *const ogg_page) -> c_long;
    pub fn ogg_page_packets(og: *const ogg_page) -> c_int;

    pub fn ogg_packet_clear(op: *mut ogg_packet);
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    #[test]
    fn bindgen_test_layout_ogg_page() {
      assert_eq!(
        ::std::mem::size_of::<ogg_page>(),
        32usize,
        concat!("Size of: ", stringify!(ogg_page))
      );
      assert_eq!(
        ::std::mem::align_of::<ogg_page>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_page))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).header as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ogg_page), "::", stringify!(header))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).header_len as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(ogg_page), "::", stringify!(header_len))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).body as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(ogg_page), "::", stringify!(body))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_page>())).body_len as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(ogg_page), "::", stringify!(body_len))
      );
    }
    #[test]
    fn bindgen_test_layout_ogg_stream_state() {
      assert_eq!(
        ::std::mem::size_of::<ogg_stream_state>(),
        376usize,
        concat!("Size of: ", stringify!(ogg_stream_state))
      );
      assert_eq!(
        ::std::mem::align_of::<ogg_stream_state>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_stream_state))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_data as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(body_data))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_storage as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(body_storage))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_fill as *const _ as usize },
        12usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(body_fill))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).body_returned as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(body_returned))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_vals as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(lacing_vals))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).granule_vals as *const _ as usize },
        32usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(granule_vals))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_storage as *const _ as usize },
        40usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(lacing_storage))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_fill as *const _ as usize },
        44usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(lacing_fill))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_packet as *const _ as usize },
        48usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(lacing_packet))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).lacing_returned as *const _ as usize },
        52usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(lacing_returned))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).header as *const _ as usize },
        56usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(header))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).header_fill as *const _ as usize },
        340usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(header_fill))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).e_o_s as *const _ as usize },
        344usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(e_o_s))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).b_o_s as *const _ as usize },
        348usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(b_o_s))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).serialno as *const _ as usize },
        352usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(serialno))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).pageno as *const _ as usize },
        356usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(pageno))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).packetno as *const _ as usize },
        360usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(packetno))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_stream_state>())).granulepos as *const _ as usize },
        368usize,
        concat!("Offset of field: ", stringify!(ogg_stream_state), "::", stringify!(granulepos))
      );
    }
    #[test]
    fn bindgen_test_layout_ogg_packet() {
      assert_eq!(
        ::std::mem::size_of::<ogg_packet>(),
        40usize,
        concat!("Size of: ", stringify!(ogg_packet))
      );
      assert_eq!(
        ::std::mem::align_of::<ogg_packet>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_packet))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).packet as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(packet))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).bytes as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(bytes))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).b_o_s as *const _ as usize },
        12usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(b_o_s))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).e_o_s as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(e_o_s))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).granulepos as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(granulepos))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_packet>())).packetno as *const _ as usize },
        32usize,
        concat!("Offset of field: ", stringify!(ogg_packet), "::", stringify!(packetno))
      );
    }
    #[test]
    fn bindgen_test_layout_oggpack_buffer() {
      assert_eq!(
        ::std::mem::size_of::<oggpack_buffer>(),
        32usize,
        concat!("Size of: ", stringify!(oggpack_buffer))
      );
      assert_eq!(
        ::std::mem::align_of::<oggpack_buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(oggpack_buffer))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).endbyte as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(oggpack_buffer), "::", stringify!(endbyte))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).endbit as *const _ as usize },
        4usize,
        concat!("Offset of field: ", stringify!(oggpack_buffer), "::", stringify!(endbit))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).buffer as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(oggpack_buffer), "::", stringify!(buffer))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).ptr as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(oggpack_buffer), "::", stringify!(ptr))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<oggpack_buffer>())).storage as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(oggpack_buffer), "::", stringify!(storage))
      );
    }
    #[test]
    fn bindgen_test_layout_ogg_iovec_t() {
      assert_eq!(
        ::std::mem::size_of::<ogg_iovec_t>(),
        16usize,
        concat!("Size of: ", stringify!(ogg_iovec_t))
      );
      assert_eq!(
        ::std::mem::align_of::<ogg_iovec_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_iovec_t))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_iovec_t>())).iov_base as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ogg_iovec_t), "::", stringify!(iov_base))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_iovec_t>())).iov_len as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(ogg_iovec_t), "::", stringify!(iov_len))
      );
    }
    #[test]
    fn bindgen_test_layout_ogg_sync_state() {
      assert_eq!(
        ::std::mem::size_of::<ogg_sync_state>(),
        32usize,
        concat!("Size of: ", stringify!(ogg_sync_state))
      );
      assert_eq!(
        ::std::mem::align_of::<ogg_sync_state>(),
        8usize,
        concat!("Alignment of ", stringify!(ogg_sync_state))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).data as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(data))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).storage as *const _ as usize },
        8usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(storage))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).fill as *const _ as usize },
        12usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(fill))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).returned as *const _ as usize },
        16usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(returned))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).unsynced as *const _ as usize },
        20usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(unsynced))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).headerbytes as *const _ as usize },
        24usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(headerbytes))
      );
      assert_eq!(
        unsafe { &(*(::std::ptr::null::<ogg_sync_state>())).bodybytes as *const _ as usize },
        28usize,
        concat!("Offset of field: ", stringify!(ogg_sync_state), "::", stringify!(bodybytes))
      );
    }
  }
}
