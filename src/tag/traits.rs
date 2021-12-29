use super::reading_mode::TextEncoding;

pub trait ToU32 {
    fn u32_from_be(&self) -> Option<u32>;
    fn u32_from_le(&self) -> Option<u32>;
}
pub trait ToU16 {
    fn u16_from_be(&self) -> Option<u16>;
    fn u16_from_le(&self) -> Option<u16>;
}

pub(crate) trait TagSize {
    fn size(&self) -> u32;
}

pub(super) trait FrameSize {
    fn total_size(&self) -> u32;
}

pub(crate) trait RawSize {
    /// Return the footprint of the struct
    /// For the Vec<Items> :
    /// raw_size = sizeof Item * Vector's length
    fn raw_size(&self) -> usize;

    fn raw_bytes(&self) -> Vec<u8>;
}

pub(crate) trait ToBytes{
    fn to_bytes(&self, text_encoding : &TextEncoding, null_terminated : bool) -> Vec<u8>;
}

pub (crate) trait StringConvert {
    fn into_string(&self, encoding : &TextEncoding) -> Option<String>;
    fn to_utf8(&self) -> Option<String>;
    fn to_uft16_le(&self) -> Option<String>;
    fn to_utf16_be(&self) -> Option<String>;
    fn first_matched_string(&mut self, encoding: &TextEncoding, drain : bool) -> Option<String>;
}

pub(crate) trait SliceConvert {
    fn to_u16_le(&self) -> Vec<u16>;
    fn to_u16_be(&self) -> Vec<u16>;
}

pub(crate) trait SplitString {
    fn split_to_string(&self, encoding: &TextEncoding) -> Vec<String>;
}

pub (crate) trait SplitUF8 {
    fn split_to_string_utf8(&self) -> Vec<String>;
}

pub (crate) trait SplitUF16 {
    fn split_to_string_utf16(&self) -> Vec<String>;
}