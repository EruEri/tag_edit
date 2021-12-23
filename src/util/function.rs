use std::iter::FromIterator;

use super::reading_mode::{TextEncoding, NULL_TERMINATE};


const LSBYTE_MASK : u16 = 0x00FF;

pub (crate) fn unsynchsafe(input : u32) -> u32 {
    let mut out : u32 = 0;
    let mut mask : u32 = 0x7F000000;

    while mask != 0 {
        out >>= 1;
        out |= input & mask;
        mask >>= 8;
    }
    out
}

pub (crate) fn synchsafe(input : u32) -> u32 {
    let mut input_copie = input;
    let mut out : u32 = 0x7F;
    let mut mask : u32 = out;

    while (mask ^ 0x7FFFFFFF) != 0 {
        out = input_copie & !mask;
        out <<= 1;
        out |= input_copie & mask;
        mask = ((mask + 1) << 8) - 1;
        input_copie = out;
    }
    out
}

/// Convert a string into a Vector of u8
/// String are cloned in the function
// pub(crate) fn string_to_vec(string: &String, encoding: TextEncoding, null_terminated : bool) -> Vec<u8> {
//     if encoding.is_one_byte() {
//         let mut vec = string.clone().into_bytes();
//         if null_terminated {
//             vec.push(NULL_TERMINATE);
//         }
//         vec 
//     }else {
//         let mut vec = vecu16_to_vecu8(encoding, string.clone().encode_utf16().collect());
//         if null_terminated {
//             vec.push(NULL_TERMINATE);
//             vec.push(NULL_TERMINATE);
//         }
//         vec
//     }
// } 
// fn vecu16_to_vecu8(encoding : TextEncoding, vec : Vec<u16>) -> Vec<u8> {
//     let mut result = vec![];
//     match encoding {
//         TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => unreachable!("Not supposed to be called with an 1 byte encoding"),
//         TextEncoding::UnicodeUtf16 => {
//             for short in vec.iter() {
//                 let msbyte = (*short >> 8) as u8;
//                 let lsbyte = (short & LSBYTE_MASK) as u8;
//                 result.push(lsbyte);
//                 result.push(msbyte);
//             }
//             result
//         },
//         TextEncoding::UnicodeBigEndian => {
//             for short in vec.iter() {
//                 let msbyte = ((*short) >> 8) as u8;
//                 let lsbyte = (short & LSBYTE_MASK) as u8;
//                 result.push(msbyte);
//                 result.push(lsbyte); 
//             }
//             result
//         }
//     }
// }

pub(crate) fn vec_to_string(mut vec :Vec<u8>, text_encoding : &TextEncoding) -> Option<String>{
    let vec_len  = vec.len() as u32;
    match text_encoding  {
        TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8  => String::from_utf8(vec).ok(),
        TextEncoding::UnicodeUtf16 => vec_to_uft16_le(&mut vec, vec_len),
        TextEncoding::UnicodeBigEndian => vec_to_utf16_be(&mut vec, vec_len),
    }
} 


pub (crate) fn vec_to_uft16_le(buffer : &mut Vec<u8>, size : u32) -> Option<String>{
    let buf = Vec::from_iter( buffer.drain(0..(size as usize)).
                            collect::<Vec<u8>>())
                            .chunks_exact(2).
                            into_iter()
                            .map(|a| u16::from_le_bytes([a[0], a[1]])).collect::<Vec<u16>>();
    let  s = String::from_utf16(&buf.as_slice()).ok()?;
    Some(s)
}
pub(crate) fn vec_to_utf16_be(buffer : &mut Vec<u8>, size : u32) -> Option<String>{
    let buf = Vec::from_iter( buffer.drain(0..(size as usize)).
    collect::<Vec<u8>>())
    .chunks_exact(2).
    into_iter()
    .map(|a| u16::from_be_bytes([a[0], a[1]])).collect::<Vec<u16>>();
    let s = String::from_utf16(&buf.as_slice()).ok()?;
    Some(s)
}

pub(crate) fn to_u16_le(buffer : &Vec<u8>) -> Vec<u16>{
    buffer.
    chunks_exact(2).
    into_iter()
    .map(|a| u16::from_le_bytes([a[0], a[1]])).collect::<Vec<u16>>()
}

pub(crate) fn to_u16_be(buffer : &Vec<u8>) -> Vec<u16>{
    buffer.
    chunks_exact(2).
    into_iter()
    .map(|a| u16::from_be_bytes([a[0], a[1]])).collect::<Vec<u16>>()
}

pub(crate) fn first_string(buffer : &mut Vec<u8>, text_encoding : &TextEncoding, drain: bool) -> Option<String>{
        let mut string_vec = vec![];
        for (i,byte) in buffer.iter().enumerate() {
            string_vec.push(*byte);
            if *byte == 0 && text_encoding.is_one_byte(){
                break;
            }else if *byte == 0 && text_encoding.encoding_size() == 2{
                let next_byte = buffer[i+1usize];
                if next_byte == 0 {
                    string_vec.push(next_byte);
                    break;
                }
            }
        }
    if drain {
        buffer.drain(0..string_vec.len());
    }
    match text_encoding {
        TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => String::from_utf8(string_vec).ok(),
        TextEncoding::UnicodeUtf16 =>  String::from_utf16(&to_u16_le(&string_vec).as_slice()).ok(),
        TextEncoding::UnicodeBigEndian => String::from_utf16(&to_u16_be(&string_vec).as_slice()).ok(),
    }
}

pub (crate) fn split_to_string_utf8(buffer : &Vec<u8>) -> Vec<String> {
    let splits = buffer.split(|n| *n == 0);
    splits.into_iter().filter(|s|  !s.is_empty()).map(|s| String::from_utf8(s.into()).unwrap()).collect()
}

pub (crate) fn split_to_string_utf16(buffer : &Vec<u16>) -> Vec<String> {
    let splits = buffer.split(|n| *n == 0);
    splits.into_iter().filter(|s|  !s.is_empty()).map(|s| String::from_utf16_lossy(s.into())).collect()
}

pub(crate) trait ToBytes{
    fn to_bytes(&self, text_encoding : &TextEncoding, null_terminated : bool) -> Vec<u8>;
}

impl ToBytes for String {
    fn to_bytes(&self, encoding : &TextEncoding, null_terminated : bool) -> Vec<u8> {
        let mut result = vec![];
        match encoding {
            TextEncoding::Iso8859_1 | TextEncoding::UnicodeUtf8 => {
                self.clone().into_bytes();
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
            result.push(NULL_TERMINATE);
            if !encoding.is_one_byte() {
                result.push(NULL_TERMINATE);
            }
        }
        result
    }
    
}

pub trait ToU32 {
    fn to_u32_be(&self) -> Option<u32>;
}
pub trait ToU16 {
    fn to_u16_be(&self) -> Option<u16>;
}

impl ToU32 for Vec<u8> {
    fn to_u32_be(&self) -> Option<u32> {
        if self.len() < 3 {
            None
        }else {
            Some(
                u32::from_be_bytes([self.get(0).unwrap().clone(), self.get(1).unwrap().clone(),
                self.get(2).unwrap().clone(), self.get(3).unwrap().clone()
                ])
            )
        }
    }
}

impl ToU16 for Vec<u8> {
    fn to_u16_be(&self) -> Option<u16> {
        if self.len() < 1 {
            None
        }else {
            Some(
                u16::from_be_bytes([self.get(0).unwrap().clone(), self.get(1).unwrap().clone()])
            )
        }
    }
}
