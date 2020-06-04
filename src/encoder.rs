
use crate::ffi::*;
use std::mem::{zeroed};
use std::ptr::null;
use av_data::frame::{Frame, MediaKind};
use av_data::pixel::{Formaton, formats};

pub struct SvtAv1Encoder{
    encoder_handle: *mut EbComponentType,
}

pub struct SvtAv1EncoderConfig {
    pub config: EbSvtAv1EncConfiguration,
    encoder_handle: *mut EbComponentType
}

struct SvtBufferHeader {
    header: EbBufferHeaderType
}

fn map_formaton(header: &mut SvtBufferHeader, fmt: &Formaton){
    match fmt {
        formats::YUV420 => {
            //TODO set bit depth of header to 8
        },
        formats::YUV420_10 => {
            //TODO set bit depth of header to 10
        },
        _ => {
            unimplemented!()
        }
    }

}

impl SvtBufferHeader {
    pub fn from_frame(frame: &Frame) -> SvtBufferHeader {
        let mut buffer:SvtBufferHeader = unsafe { zeroed() };
        if let MediaKind::Video(ref v) = frame.kind {
            map_formaton(&mut buffer, &v.format)
            //TODO set buffer.pic_type ( cant interpret currently as no av-data documentation on picture types)
            //TODO set other buffer info (qp m luma sse cr_sse cb_sse flags luma_ssim cr_ssim cb_ssim)
        }
        buffer.dts = frame.t.dts.or_else(0);
        buffer.pts = frame.t.pts.or_else(0);
        //TODO set other buffer info (p_buffer size n_filled_len n_alloc_len p_app_private wrapper_ptr n_tick_count)
        return buffer;
    }
}

impl SvtAv1EncoderConfig {
    pub fn new(width: u32, height: u32) -> Result<SvtAv1EncoderConfig, EbErrorType::Type>  {
        //library initialises both these values
        let mut cfg: EbSvtAv1EncConfiguration = unsafe { zeroed() };
        let mut handle: *mut EbComponentType = unsafe{ zeroed() };

        //initialise handle and set default values for config
        let ret: EbErrorType::Type = unsafe {
            svt_av1_enc_init_handle(&mut handle as *mut *mut EbComponentType,
                                    null::<std::ffi::c_void>() as *mut std::ffi::c_void,
                                    &mut cfg as *mut EbSvtAv1EncConfiguration)
        };
        if ret != EbErrorType::EB_ErrorNone {
            return Err(ret)
        }
        cfg.source_width = width;
        cfg.source_height = height;

        return Ok(SvtAv1EncoderConfig { config: cfg, encoder_handle: handle})
    }

    pub fn create_encoder(self) -> Result<SvtAv1Encoder, EbErrorType::Type>{
        return SvtAv1Encoder::new(self)
    }
}

impl SvtAv1Encoder {

    pub fn new(mut cfg: SvtAv1EncoderConfig) -> Result<SvtAv1Encoder, EbErrorType::Type> {
        assert!( cfg.config.source_width>=64 && cfg.config.source_height >= 64);
        //configure encoder
        let ret = unsafe{
            svt_av1_enc_set_parameter(cfg.encoder_handle, &mut cfg.config as *mut EbSvtAv1EncConfiguration)
        };
        if ret != EbErrorType::EB_ErrorNone {
            return Result::Err(ret)
        }

        //initialise encoder
        let ret = unsafe{
            svt_av1_enc_init(cfg.encoder_handle)
        };
        if ret != EbErrorType::EB_ErrorNone {
            return Result::Err(ret)
        }
        return Result::Ok(SvtAv1Encoder { encoder_handle: cfg.encoder_handle});
    }

    pub fn encode(&mut self, frame: &Frame) -> Result<(), EbErrorType::Type> {
        let mut buffer = SvtBufferHeader::from_frame(frame);
        let result = unsafe {
            svt_av1_enc_send_picture(self.encoder_handle, &mut buffer.header)
        };
        return match result {
            EbErrorType::EB_ErrorNone => Ok(()),
            e => Err(e)
        }
    }

}

impl Drop for SvtAv1Encoder {
    fn drop(&mut self) {
        unsafe {
            let result1 = svt_av1_enc_deinit(self.encoder_handle);
            let result2 = svt_av1_enc_deinit_handle(self.encoder_handle);
            assert!(result1 == EbErrorType::EB_ErrorNone);
            assert!(result2 == EbErrorType::EB_ErrorNone);
        }
    }
}

unsafe impl Send for SvtAv1Encoder{} //TODO check this cant be abused

#[cfg(feature = "codec-trait")]
mod encoder_trait {
    use super::*;
    use crate::codec::encoder::*;
    use crate::codec::error::*;
    use crate::data::value::Value;
    use crate::data::params::CodecParams;
    use crate::data::frame::ArcFrame;
    use crate::data::packet::Packet;

    impl Encoder for SvtAv1Encoder {
        fn get_extradata(&self) -> Option<Vec<u8>> {
            unimplemented!()
        }

        fn send_frame(&mut self, pkt: &ArcFrame) -> Result<()> {
            unimplemented!()
        }

        fn receive_packet(&mut self) -> Result<Packet> {
            unimplemented!()
        }

        fn flush(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn configure(&mut self) -> Result<()> {
            unimplemented!()
        }

        fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()> {
            unimplemented!()
        }

        fn set_params(&mut self, params: &CodecParams) -> Result<()> {
            unimplemented!()
        }

        fn get_params(&self) -> Result<CodecParams> {
            unimplemented!()
        }
    }
}