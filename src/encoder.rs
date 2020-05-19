
use crate::ffi::*;
use std::marker::PhantomData;
use std::mem::{zeroed};
use std::ptr::null;

pub struct SvtAv1Encoder<T>{
    //encoder_handle: *mut EbComponentType,
    private_data: PhantomData<T>,
}

impl<T> SvtAv1Encoder<T> {

    pub fn new() -> Result<SvtAv1Encoder<T>, EbErrorType::Type> {
        //library initialises both these values
        let mut handle: *mut EbComponentType = unsafe{ zeroed() };
        let mut cfg: EbSvtAv1EncConfiguration = unsafe { zeroed() };

        //initialise handle and set default values for config
        let ret: EbErrorType::Type = unsafe {
            svt_av1_enc_init_handle(&mut handle as *mut *mut EbComponentType,
                                    null::<std::ffi::c_void>() as *mut std::ffi::c_void,
                                    &mut cfg as *mut EbSvtAv1EncConfiguration)
        };
        if ret != EbErrorType::EB_ErrorNone {
            return Result::Err(ret)
        }

        //configure encoder
        let ret = unsafe{
            svt_av1_enc_set_parameter(handle, &mut cfg as *mut EbSvtAv1EncConfiguration)
        };

        //initialise encoder
        let ret = unsafe{
            svt_av1_enc_init(handle)
        };

        return Result::Ok(SvtAv1Encoder { private_data: PhantomData });
        //return Result::Ok(SvtAv1Encoder { encoder_handle: handle, private_data: PhantomData });
    }

}

#[cfg(feature = "codec-trait")]
mod encoder_trait {
    use super::*;
    use crate::codec::encoder::*;
    use crate::codec::error::*;
    use crate::data::value::Value;
    use crate::data::params::CodecParams;
    use crate::data::frame::ArcFrame;
    use crate::data::packet::Packet;
    use crate::data::timeinfo::TimeInfo;

    impl Encoder for SvtAv1Encoder<TimeInfo> {
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