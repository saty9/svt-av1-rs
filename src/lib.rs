#![allow(non_upper_case_globals)]

extern crate svt_av1_sys as ffi;
extern crate av_data as data;

#[cfg(feature = "codec-trait")]
extern crate av_codec as codec;

pub mod encoder;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::EbErrorType;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_create_new_encoder() -> Result<(), EbErrorType::Type> {
        let mut cfg_result = encoder::SvtAv1EncoderConfig::new(64,64);
        let mut cfg = match cfg_result {
            Ok(e) => e,
            Err(e) => return Err(e)
        };
        let enc = cfg.create_encoder();
        match enc {
            Ok(e) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
