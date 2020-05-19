#![allow(non_upper_case_globals)]

extern crate svt_av1_sys as ffi;
extern crate av_data as data;

#[cfg(feature = "codec-trait")]
extern crate av_codec as codec;

pub mod encoder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_create_new_encoder() {
        let enc = encoder::SvtAv1Encoder::<i32>::new();
    }
}
