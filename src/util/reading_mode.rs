pub(crate) const NULL_TERMINATE : u8 = 0;

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum TextEncoding {
    Iso8859_1 = 0x00,
    UnicodeUtf16 = 0x01,
    UnicodeBigEndian = 0x02,
    UnicodeUtf8 = 0x03
}

impl TextEncoding {
    pub fn from_raw_value(value : u8) -> Option<TextEncoding>{
        match value {
            0x00 => Some (TextEncoding::Iso8859_1),
            0x01 => Some( TextEncoding::UnicodeUtf16),
            0x02 => Some(TextEncoding::UnicodeBigEndian),
            0x03 => Some(TextEncoding::UnicodeUtf8),
            _ => None
        }
    }
    pub fn encoding_size(&self) -> u8 {
        match self {
            TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => 1,
            TextEncoding::UnicodeUtf16 | TextEncoding::UnicodeBigEndian => 2,
        }
    }
    pub fn is_one_byte(&self) -> bool {
        match self {
            TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => true,
            TextEncoding::UnicodeUtf16 | TextEncoding::UnicodeBigEndian => false,
        }
    }
}