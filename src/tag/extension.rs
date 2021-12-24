use crate::util::function::LSBYTE_MASK;

use super::{traits::{SliceConvert, StringConvert, SplitUF8, SplitUF16, ToBytes, SplitString}, reading_mode::{TextEncoding, NULL_TERMINATE}};


impl ToBytes for String {
    fn to_bytes(&self, encoding : &TextEncoding, null_terminated : bool) -> Vec<u8> {
        if self == "\u{0}\u{0}" || self == "\u{0}" {
            return self.clone().into_bytes();
        }
        
        let mut result = vec![];
        match encoding {
            TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => {
                result = self.clone().into_bytes();
            },
            TextEncoding::UnicodeUtf16 => {
                let vec : Vec<u16> = self.clone().encode_utf16().collect();
                for short in vec.iter() {
                    let msbyte = (*short >> 8) as u8;
                    let lsbyte = (short & LSBYTE_MASK) as u8;
                    result.push(lsbyte);
                    result.push(msbyte);
                }
            },
            TextEncoding::UnicodeBigEndian => {
                let vec : Vec<u16> = self.clone().encode_utf16().collect();
                for short in vec.iter() {
                    let msbyte = ((*short) >> 8) as u8;
                    let lsbyte = (short & LSBYTE_MASK) as u8;
                    result.push(msbyte);
                    result.push(lsbyte); 
                }
            }
        }
        if null_terminated {
            if encoding.is_one_byte() && !self.ends_with("\u{0}"){
                result.push(NULL_TERMINATE);
            }else if !encoding.is_one_byte() && !self.ends_with("\u{0}\u{0}"){
                result.push(NULL_TERMINATE);
                result.push(NULL_TERMINATE);
            }
        }
        result
    }
    
}



impl SliceConvert for Vec<u8> {
    fn to_u16_le(&self) -> Vec<u16> {
        self.chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect::<Vec<u16>>()
    }

    fn to_u16_be(&self) -> Vec<u16> {
        self.chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_be_bytes([a[0], a[1]]))
            .collect::<Vec<u16>>()
    }
}

impl SplitUF8 for Vec<u8> {
    fn split_to_string_utf8(&self) -> Vec<String> {
        let splits = self.split(|n| *n == 0);
        splits
        .into_iter()
        .filter(|s|  !s.is_empty())
        .map(|s| String::from_utf8(s.into())
        .unwrap())
        .collect()
    }
}

impl SplitUF16 for Vec<u16> {
    fn split_to_string_utf16(&self) -> Vec<String>{
        let splits = self.split(|n| *n == 0);
        splits
        .into_iter()
        .filter(|s|  !s.is_empty()).map(|s| String::from_utf16_lossy(s.into()))
        .collect()
    }
}

impl SplitString for Vec<u8> {
    fn split_to_string(&self, encoding: &TextEncoding) -> Vec<String> {
        match encoding {
            TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => self.split_to_string_utf8(),
            TextEncoding::UnicodeUtf16 => self.to_u16_le().split_to_string_utf16(),
            TextEncoding::UnicodeBigEndian => self.to_u16_be().split_to_string_utf16(),
        }
    }
}

impl StringConvert for Vec<u8> {

    fn into_string(&self, encoding : &super::reading_mode::TextEncoding) -> Option<String> {
            match encoding {
                TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => self.to_utf8(),
                TextEncoding::UnicodeUtf16 => self.to_uft16_le(),
                TextEncoding::UnicodeBigEndian => self.to_utf16_be(),
            }
    }
    fn to_utf8(&self) -> Option<String> {
        String::from_utf8(self.clone()).ok()
    }

    fn to_uft16_le(&self) -> Option<String> {
        let buf = self
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_le_bytes([a[0], a[1]]))
            .collect::<Vec<u16>>();
        let s = String::from_utf16(&buf.as_slice()).ok()?;
        Some(s)
    }

    fn to_utf16_be(&self) -> Option<String> {
        let buf = self
            .chunks_exact(2)
            .into_iter()
            .map(|a| u16::from_be_bytes([a[0], a[1]]))
            .collect::<Vec<u16>>();
        let s = String::from_utf16(&buf.as_slice()).ok()?;
        Some(s)
    }

    fn first_matched_string(
        &mut self,  encoding: &super::reading_mode::TextEncoding,
        drain: bool) -> Option<String> {
        let mut string_vec = vec![];
        for (i, byte) in self.iter().enumerate() {
            string_vec.push(*byte);
            if *byte == 0 && encoding.is_one_byte() {
                break;
            } else if *byte == 0 && encoding.encoding_size() == 2 {
                let next_byte = self[i + 1usize];
                if next_byte == 0 {
                    string_vec.push(next_byte);
                    break;
                }
            }
        }
        if drain {
            self.drain(0..string_vec.len());
        }
        string_vec.into_string(encoding)
    }
}
