#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum TextEncoding {
    ISO_8859_1 = 0x00,
    Unicode_UTF_16 = 0x01,
    Unicode_BigEndian = 0x02,
    Unicode_UTF_8 = 0x03
}

impl TextEncoding {
    pub fn from_raw_value(value : u8) -> Option<TextEncoding>{
        match value {
            0x00 => Some (TextEncoding::ISO_8859_1),
            0x01 => Some( TextEncoding::Unicode_UTF_16),
            0x02 => Some(TextEncoding::Unicode_BigEndian),
            0x03 => Some(TextEncoding::Unicode_UTF_8),
            _ => None
        }
    }
    pub fn encoding_size(&self) -> u8 {
        match self {
            TextEncoding::ISO_8859_1 | TextEncoding::Unicode_UTF_8 => 1,
            TextEncoding::Unicode_UTF_16 | TextEncoding::Unicode_BigEndian => 2,
        }
    }
    pub fn is_one_byte(&self) -> bool {
        match self {
            TextEncoding::ISO_8859_1 | TextEncoding::Unicode_UTF_8 => true,
            TextEncoding::Unicode_UTF_16 | TextEncoding::Unicode_BigEndian => false,
        }
    }
}