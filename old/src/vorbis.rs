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
}

#[allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]
pub mod codec {
  use crate::ogg::*;
  use std::os::raw::*;

  /* Vorbis ERRORS and return codes ***********************************/

  pub const OV_FALSE: i32 = -1;
  pub const OV_EOF: i32 = -2;
  pub const OV_HOLE: i32 = -3;
  pub const OV_EREAD: i32 = -128;
  pub const OV_EFAULT: i32 = -129;
  pub const OV_EIMPL: i32 = -130;
  pub const OV_EINVAL: i32 = -131;
  pub const OV_ENOTVORBIS: i32 = -132;
  pub const OV_EBADHEADER: i32 = -133;
  pub const OV_EVERSION: i32 = -134;
  pub const OV_ENOTAUDIO: i32 = -135;
  pub const OV_EBADPACKET: i32 = -136;
  pub const OV_EBADLINK: i32 = -137;
  pub const OV_ENOSEEK: i32 = -138;

  // vorbis_info contains all the setup information specific to the specific compression/decompression mode in progress
  // (eg, psychoacoustic settings, channel setup, options, codebook, etc).
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct vorbis_info {
    pub version: c_int,
    pub channels: c_int,
    pub rate: c_long,

    /*
      The below bitrate declarations are *hints*.
      Combinations of the three values carry the following implications:

      all three set to the same value:
        implies a fixed rate bitstream
      only nominal set:
        implies a VBR stream that averages the nominal bitrate. No hard upper/lower limit
      upper and or lower set:
        implies a VBR bitstream that obeys the bitrate limits. nominal may also be set to give a nominal rate.
      none set:
        the coder does not care to speculate.
    */
    pub bitrate_upper: c_long,
    pub bitrate_nominal: c_long,
    pub bitrate_lower: c_long,
    pub bitrate_window: c_long,

    pub codec_setup: *mut c_void,
  }

  // vorbis_dsp_state buffers the current vorbis audio analysis/synthesis state.
  // The DSP state belongs to a specific logical bitstream.
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct vorbis_dsp_state {
    pub analysisp: c_int,
    pub vi: *mut vorbis_info,

    pub pcm: *mut *mut f32,
    pub pcmret: *mut *mut f32,
    pub pcm_storage: c_int,
    pub pcm_current: c_int,
    pub pcm_returned: c_int,

    pub preextrapolate: c_int,
    pub eofflag: c_int,

    pub lW: c_long,
    pub W: c_long,
    pub nW: c_long,
    pub centerW: c_long,

    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,

    pub glue_bits: ogg_int64_t,
    pub time_bits: ogg_int64_t,
    pub floor_bits: ogg_int64_t,
    pub res_bits: ogg_int64_t,

    pub backend_state: *mut c_void,
  }

  // vorbis_block is a single block of data to be processed as part of the analysis/synthesis stream;
  // it belongs to a specific logical bitstream, but is independent from other vorbis_blocks
  // belonging to that logical bitstream.
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct vorbis_block {
    pub pcm: *mut *mut f32, // this is a pointer into local storage
    pub opb: oggpack_buffer,

    pub lW: c_long,
    pub W: c_long,
    pub nW: c_long,
    pub pcmend: c_int,
    pub mode: c_int,

    pub eofflag: c_int,
    pub granulepos: ogg_int64_t,
    pub sequence: ogg_int64_t,
    pub vd: *mut vorbis_dsp_state, // For read-only access of configuration

    // local storage to avoid remallocing; it's up to the mapping to structure it
    pub localstore: *mut c_void,
    pub localtop: c_long,
    pub localalloc: c_long,
    pub totaluse: c_long,
    pub reap: *mut alloc_chain,

    // bitmetrics for the frame
    pub glue_bits: c_long,
    pub time_bits: c_long,
    pub floor_bits: c_long,
    pub res_bits: c_long,

    pub internal: *mut c_void,
  }

  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct alloc_chain {
    pub ptr: *mut c_void,
    pub next: *mut alloc_chain,
  }

  // the comments are not part of vorbis_info so that vorbis_info can be static storage
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct vorbis_comment {
    pub user_comments: *mut *mut c_char, // unlimited user comment fields. libvorbis writes 'libvorbis' whatever vendor is set to in encode
    pub comment_lengths: *mut c_int,
    pub comments: c_int,
    pub vendor: *mut c_char,
  }

  // libvorbis encodes in two abstraction layers; first we perform DSP
  // and produce a packet (see docs/analysis.txt).  The packet is then
  // coded into a framed OggSquish bitstream by the second layer (see
  // docs/framing.txt).  Decode is the reverse process; we sync/frame
  // the bitstream and extract individual packets, then decode the
  // packet back into PCM audio.
  //
  // The extra framing/packetizing is used in streaming formats, such as
  // files.  Over the net (such as with UDP), the framing and
  // packetization aren't necessary as they're provided by the transport
  // and the streaming layer is not used

  #[link(name = "libvorbis", kind = "static")]
  extern "C" {
    /* Vorbis PRIMITIVES: general ***************************************/

    pub fn vorbis_info_init(vi: *mut vorbis_info);
    pub fn vorbis_info_clear(vi: *mut vorbis_info);
    pub fn vorbis_info_blocksize(vi: *mut vorbis_info, zo: c_int) -> c_int;
    pub fn vorbis_comment_init(vc: *mut vorbis_comment);
    pub fn vorbis_comment_add(vc: *mut vorbis_comment, comment: *const c_char);
    pub fn vorbis_comment_add_tag(vc: *mut vorbis_comment, tag: *const c_char, contents: *const c_char);
    pub fn vorbis_comment_query(vc: *mut vorbis_comment, tag: *const c_char, count: c_int) -> *mut c_char;
    pub fn vorbis_comment_query_count(vc: *mut vorbis_comment, tag: *const c_char) -> c_int;
    pub fn vorbis_comment_clear(vc: *mut vorbis_comment);

    pub fn vorbis_block_init(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_block_clear(vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_dsp_clear(v: *mut vorbis_dsp_state);
    pub fn vorbis_granule_time(v: *mut vorbis_dsp_state, granulepos: ogg_int64_t) -> f64;

    pub fn vorbis_version_string() -> *const c_char;

    /* Vorbis PRIMITIVES: analysis/DSP layer ****************************/

    pub fn vorbis_analysis_init(v: *mut vorbis_dsp_state, vi: *mut vorbis_info) -> c_int;
    pub fn vorbis_commentheader_out(vc: *mut vorbis_comment, op: *mut ogg_packet) -> c_int;
    pub fn vorbis_analysis_headerout(
      v: *mut vorbis_dsp_state,
      vc: *mut vorbis_comment,
      op: *mut ogg_packet,
      op_comm: *mut ogg_packet,
      op_code: *mut ogg_packet,
    ) -> c_int;
    pub fn vorbis_analysis_buffer(v: *mut vorbis_dsp_state, vals: c_int) -> *mut *mut f32;
    pub fn vorbis_analysis_wrote(v: *mut vorbis_dsp_state, vals: c_int) -> c_int;
    pub fn vorbis_analysis_blockout(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_analysis(vb: *mut vorbis_block, op: *mut ogg_packet) -> c_int;

    pub fn vorbis_bitrate_addblock(vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_bitrate_flushpacket(vd: *mut vorbis_dsp_state, op: *mut ogg_packet) -> c_int;

    /* Vorbis PRIMITIVES: synthesis layer *******************************/

    pub fn vorbis_synthesis_idheader(op: *mut ogg_packet) -> c_int;
    pub fn vorbis_synthesis_headerin(vi: *mut vorbis_info, vc: *mut vorbis_comment, op: *mut ogg_packet) -> c_int;

    pub fn vorbis_synthesis_init(v: *mut vorbis_dsp_state, vi: *mut vorbis_info) -> c_int;
    pub fn vorbis_synthesis_restart(v: *mut vorbis_dsp_state) -> c_int;
    pub fn vorbis_synthesis(vb: *mut vorbis_block, op: *mut ogg_packet) -> c_int;
    pub fn vorbis_synthesis_trackonly(vb: *mut vorbis_block, op: *mut ogg_packet) -> c_int;
    pub fn vorbis_synthesis_blockin(v: *mut vorbis_dsp_state, vb: *mut vorbis_block) -> c_int;
    pub fn vorbis_synthesis_pcmout(v: *mut vorbis_dsp_state, pcm: *mut *mut *mut f32) -> c_int;
    pub fn vorbis_synthesis_lapout(v: *mut vorbis_dsp_state, pcm: *mut *mut *mut f32) -> c_int;
    pub fn vorbis_synthesis_read(v: *mut vorbis_dsp_state, samples: c_int) -> c_int;
    pub fn vorbis_packet_blocksize(vi: *mut vorbis_info, op: *mut ogg_packet) -> c_long;

    pub fn vorbis_synthesis_halfrate(v: *mut vorbis_info, flag: c_int) -> c_int;
    pub fn vorbis_synthesis_halfrate_p(v: *mut vorbis_info) -> c_int;
  }
}

// Libvorbisenc is a convenient API for setting up an encoding environment using libvorbis.
// Libvorbisenc encapsulates the actions needed to set up the encoder properly.
#[allow(dead_code, non_camel_case_types, non_snake_case, unused_variables)]
mod enc {
  use crate::vorbis::codec::*;
  use std::os::raw::*;

  // vorbis_encode_ctl() codes

  pub const OV_ECTL_RATEMANAGE2_GET: u32 = 20; // Query the current encoder bitrate management setting
  pub const OV_ECTL_RATEMANAGE2_SET: u32 = 21; // Set the current encoder bitrate management settings
  pub const OV_ECTL_LOWPASS_GET: u32 = 32; // Returns the current encoder hard-lowpass setting (kHz) in the double pointed to by arg
  pub const OV_ECTL_LOWPASS_SET: u32 = 33; /* Sets the encoder hard-lowpass to the value (kHz) pointed to by arg.
                                           Valid lowpass settings range from 2 to 99. */
  pub const OV_ECTL_IBLOCK_GET: u32 = 48; // Returns the current encoder impulse block setting in the double pointed to by arg
  pub const OV_ECTL_IBLOCK_SET: u32 = 49; // Sets the impulse block bias to the the value pointed to by arg.
  pub const OV_ECTL_COUPLING_GET: u32 = 64; // Returns the current encoder coupling setting in the int pointed to by arg
  pub const OV_ECTL_COUPLING_SET: u32 = 65; // Enables/disables channel coupling in multichannel encoding according to arg

  // deprecated rate management supported only for compatibility

  pub const OV_ECTL_RATEMANAGE_GET: u32 = 16; // Old interface to querying bitrate management settings
  pub const OV_ECTL_RATEMANAGE_SET: u32 = 17; // Old interface to modifying bitrate management settings
  pub const OV_ECTL_RATEMANAGE_AVG: u32 = 18; // Old interface to setting average-bitrate encoding mode
  pub const OV_ECTL_RATEMANAGE_HARD: u32 = 19; // Old interface to setting bounded-bitrate encoding modes

  /**
   * \deprecated This is a deprecated interface. Please use vorbis_encode_ctl()
   * with the \ref ovectl_ratemanage2_arg struct and \ref
   * OV_ECTL_RATEMANAGE2_GET and \ref OV_ECTL_RATEMANAGE2_SET calls in new code.
   *
   * The \ref ovectl_ratemanage_arg structure is used with vorbis_encode_ctl()
   * and the \ref OV_ECTL_RATEMANAGE_GET, \ref OV_ECTL_RATEMANAGE_SET, \ref
   * OV_ECTL_RATEMANAGE_AVG, \ref OV_ECTL_RATEMANAGE_HARD calls in order to
   * query and modify specifics of the encoder's bitrate management
   * configuration.
   */
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ovectl_ratemanage_arg {
    pub management_active: c_int,
    pub bitrate_hard_min: c_long,
    pub bitrate_hard_max: c_long,
    pub bitrate_hard_window: f64,
    pub bitrate_av_lo: c_long,
    pub bitrate_av_hi: c_long,
    pub bitrate_av_window: f64,
    pub bitrate_av_window_center: f64,
  }

  /**
   * \name struct ovectl_ratemanage2_arg
   *
   * The ovectl_ratemanage2_arg structure is used with vorbis_encode_ctl() and
   * the OV_ECTL_RATEMANAGE2_GET and OV_ECTL_RATEMANAGE2_SET calls in order to
   * query and modify specifics of the encoder's bitrate management
   * configuration.
   */
  #[repr(C)]
  #[derive(Debug, Copy, Clone)]
  pub struct ovectl_ratemanage2_arg {
    pub management_active: c_int, /* nonzero if bitrate management is active
                                  Lower allowed bitrate limit in kilobits per second */
    pub bitrate_limit_min_kbps: c_long,
    pub bitrate_limit_max_kbps: c_long, // Upper allowed bitrate limit in kilobits per second
    pub bitrate_limit_reservoir_bits: c_long, /* Size of the bitrate reservoir in bits
                                        Regulates the bitrate reservoir's preferred fill level in a range from 0.0
                                        to 1.0; 0.0 tries to bank bits to buffer against future bitrate spikes, 1.0
                                        buffers against future sudden drops in instantaneous bitrate. Default is
                                        0.1 */
    pub bitrate_limit_reservoir_bias: f64,
    pub bitrate_average_kbps: c_long, // Average bitrate setting in kilobits per second
    pub bitrate_average_damping: f64, /* Slew rate limit setting for average bitrate adjustment; sets the minimum
                                      time in seconds the bitrate tracker may swing from one extreme to the
                                      other when boosting or damping average bitrate.*/
  }

  #[link(name = "libvorbis", kind = "static")]
  extern "C" {
    /**
     * This is the primary function within libvorbisenc for setting up managed
     * bitrate modes.
     *
     * Before this function is called, the \ref vorbis_info
     * struct should be initialized by using vorbis_info_init() from the libvorbis
     * API.  After encoding, vorbis_info_clear() should be called.
     *
     * The max_bitrate, nominal_bitrate, and min_bitrate settings are used to set
     * constraints for the encoded file.  This function uses these settings to
     * select the appropriate encoding mode and set it up.
     *
     * \param vi               Pointer to an initialized \ref vorbis_info struct.
     * \param channels         The number of channels to be encoded.
     * \param rate             The sampling rate of the source audio.
     * \param max_bitrate      Desired maximum bitrate (limit). -1 indicates unset.
     * \param nominal_bitrate  Desired average, or central, bitrate. -1 indicates unset.
     * \param min_bitrate      Desired minimum bitrate. -1 indicates unset.
     *
     * \return                 Zero for success, and negative values for failure.
     *
     * \retval 0               Success.
     * \retval OV_EFAULT       Internal logic fault; indicates a bug or heap/stack corruption.
     * \retval OV_EINVAL       Invalid setup request, eg, out of range argument.
     * \retval OV_EIMPL        Unimplemented mode; unable to comply with bitrate request.
     */
    pub fn vorbis_encode_init(
      vi: *mut vorbis_info,
      channels: c_long,
      rate: c_long,
      max_bitrate: c_long,
      nominal_bitrate: c_long,
      min_bitrate: c_long,
    ) -> c_int;

    /**
     * This function performs step-one of a three-step bitrate-managed encode
     * setup.  It functions similarly to the one-step setup performed by \ref
     * vorbis_encode_init but allows an application to make further encode setup
     * tweaks using \ref vorbis_encode_ctl before finally calling \ref
     * vorbis_encode_setup_init to complete the setup process.
     *
     * Before this function is called, the \ref vorbis_info struct should be
     * initialized by using vorbis_info_init() from the libvorbis API.  After
     * encoding, vorbis_info_clear() should be called.
     *
     * The max_bitrate, nominal_bitrate, and min_bitrate settings are used to set
     * constraints for the encoded file.  This function uses these settings to
     * select the appropriate encoding mode and set it up.
     *
     * \param vi                Pointer to an initialized vorbis_info struct.
     * \param channels          The number of channels to be encoded.
     * \param rate              The sampling rate of the source audio.
     * \param max_bitrate       Desired maximum bitrate (limit). -1 indicates unset.
     * \param nominal_bitrate   Desired average, or central, bitrate. -1 indicates unset.
     * \param min_bitrate       Desired minimum bitrate. -1 indicates unset.
     *
     * \return                  Zero for success, and negative for failure.
     *
     * \retval 0                Success
     * \retval OV_EFAULT        Internal logic fault; indicates a bug or heap/stack corruption.
     * \retval OV_EINVAL        Invalid setup request, eg, out of range argument.
     * \retval OV_EIMPL         Unimplemented mode; unable to comply with bitrate request.
     */
    pub fn vorbis_encode_setup_managed(
      vi: *mut vorbis_info,
      channels: c_long,
      rate: c_long,
      max_bitrate: c_long,
      nominal_bitrate: c_long,
      min_bitrate: c_long,
    ) -> c_int;

    /**
     * This function performs step-one of a three-step variable bitrate
     * (quality-based) encode setup.  It functions similarly to the one-step setup
     * performed by \ref vorbis_encode_init_vbr() but allows an application to
     * make further encode setup tweaks using \ref vorbis_encode_ctl() before
     * finally calling \ref vorbis_encode_setup_init to complete the setup
     * process.
     *
     * Before this function is called, the \ref vorbis_info struct should be
     * initialized by using \ref vorbis_info_init() from the libvorbis API.  After
     * encoding, vorbis_info_clear() should be called.
     *
     * \param vi           Pointer to an initialized vorbis_info struct.
     * \param channels     The number of channels to be encoded.
     * \param rate         The sampling rate of the source audio.
     * \param quality      Desired quality level, currently from -0.1 to 1.0 (lo to hi).
     *
     * \return             Zero for success, and negative values for failure.
     *
     * \retval  0          Success
     * \retval  OV_EFAULT  Internal logic fault; indicates a bug or heap/stack corruption.
     * \retval  OV_EINVAL  Invalid setup request, eg, out of range argument.
     * \retval  OV_EIMPL   Unimplemented mode; unable to comply with quality level request.
     */
    pub fn vorbis_encode_setup_vbr(vi: *mut vorbis_info, channels: c_long, rate: c_long, quality: f32) -> c_int;

    /**
     * This is the primary function within libvorbisenc for setting up variable
     * bitrate ("quality" based) modes.
     *
     *
     * Before this function is called, the vorbis_info struct should be
     * initialized by using vorbis_info_init() from the libvorbis API. After
     * encoding, vorbis_info_clear() should be called.
     *
     * \param vi           Pointer to an initialized vorbis_info struct.
     * \param channels     The number of channels to be encoded.
     * \param rate         The sampling rate of the source audio.
     * \param base_quality Desired quality level, currently from -0.1 to 1.0 (lo to hi).
     *
     * \return             Zero for success, or a negative number for failure.
     *
     * \retval 0           Success
     * \retval OV_EFAULT   Internal logic fault; indicates a bug or heap/stack corruption.
     * \retval OV_EINVAL   Invalid setup request, eg, out of range argument.
     * \retval OV_EIMPL    Unimplemented mode; unable to comply with quality level request.
     */
    pub fn vorbis_encode_init_vbr(vi: *mut vorbis_info, channels: c_long, rate: c_long, base_quality: f32) -> c_int;

    /**
     * This function performs the last stage of three-step encoding setup, as
     * described in the API overview under managed bitrate modes.
     *
     * Before this function is called, the \ref vorbis_info struct should be
     * initialized by using vorbis_info_init() from the libvorbis API, one of
     * \ref vorbis_encode_setup_managed() or \ref vorbis_encode_setup_vbr() called to
     * initialize the high-level encoding setup, and \ref vorbis_encode_ctl()
     * called if necessary to make encoding setup changes.
     * vorbis_encode_setup_init() finalizes the highlevel encoding structure into
     * a complete encoding setup after which the application may make no further
     * setup changes.
     *
     * After encoding, vorbis_info_clear() should be called.
     *
     * \param vi           Pointer to an initialized \ref vorbis_info struct.
     *
     * \return             Zero for success, and negative values for failure.
     *
     * \retval 0           Success.
     * \retval OV_EFAULT   Internal logic fault; indicates a bug or heap/stack corruption.
     *
     * \retval OV_EINVAL   Attempt to use vorbis_encode_setup_init() without first
     *                     calling one of vorbis_encode_setup_managed() or vorbis_encode_setup_vbr() to
     *                     initialize the high-level encoding setup
     */
    pub fn vorbis_encode_setup_init(vi: *mut vorbis_info) -> c_int;

    /**
     * This function implements a generic interface to miscellaneous encoder
     * settings similar to the classic UNIX 'ioctl()' system call.  Applications
     * may use vorbis_encode_ctl() to query or set bitrate management or quality
     * mode details by using one of several \e request arguments detailed below.
     * vorbis_encode_ctl() must be called after one of
     * vorbis_encode_setup_managed() or vorbis_encode_setup_vbr().  When used
     * to modify settings, \ref vorbis_encode_ctl() must be called before \ref
     * vorbis_encode_setup_init().
     *
     * \param vi         Pointer to an initialized vorbis_info struct.
     *
     * \param number     Specifies the desired action; See \ref encctlcodes "the list
     *                   of available requests".
     *
     * \param arg        void * pointing to a data structure matching the request argument.
     *
     * \retval            0 Success. Any further return information (such as the result of a
     *                    query) is placed into the storage pointed to by *arg.
     *
     * \retval OV_EINVAL  Invalid argument, or an attempt to modify a setting after
     *                    calling vorbis_encode_setup_init().
     *
     * \retval OV_EIMPL   Unimplemented or unknown request
     */
    pub fn vorbis_encode_ctl(vi: *mut vorbis_info, number: c_int, arg: *mut c_void) -> c_int;
  }
}

#[cfg(test)]
mod tests {
  #[allow(unused_imports)]
  use {
    crate::vorbis::{codec::*, enc::*, ogg::*},
    std::ffi::{CStr, CString},
    std::mem::*,
  };

  #[test]
  fn ogg_binding_ok() {
    unsafe {
      let mut t = std::mem::zeroed::<ogg_stream_state>();
      let v = ogg_stream_init(&mut t, 0);
      assert_eq!(v, 0);
    }
  }

  #[test]
  fn codec_binding_ok() {
    unsafe {
      let mut t = std::mem::zeroed::<vorbis_info>();
      vorbis_info_init(&mut t);
      let ver_cstr = CStr::from_ptr(vorbis_version_string()); // not owned string container
      let ver = ver_cstr.to_str().expect("to_str() call failed"); // ref to str in container
      assert_eq!(ver, "Xiph.Org libVorbis 1.3.6");
    }
  }

  #[test]
  fn enc_binding_ok() {
    unsafe {
      let mut t = std::mem::zeroed::<vorbis_info>();
      vorbis_info_init(&mut t);
      let init = vorbis_encode_init(&mut t, 2, 44_100, -1, 192_000, -1); // 44kHz stereo coupled, ~ 192kbps VBR
      assert_eq!(init, 0);
    }
  }
}
