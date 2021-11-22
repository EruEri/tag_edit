use std::iter::FromIterator;

use super::reading_mode::TextEncoding;

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

/*pub (crate) fn synchsafe(input : u32) -> u32 {
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
}*/

pub(crate) fn vec_to_string(mut vec :Vec<u8>, text_encoding : &TextEncoding) -> Option<String>{
    let vec_len  = vec.len() as u32;
    match text_encoding  {
        TextEncoding::ISO_8859_1 | TextEncoding::Unicode_UTF_8  => String::from_utf8(vec).ok(),
        TextEncoding::Unicode_UTF_16 => vec_to_uft16_le(&mut vec, vec_len),
        TextEncoding::Unicode_BigEndian => vec_to_utf16_be(&mut vec, vec_len),
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
        TextEncoding::ISO_8859_1 | TextEncoding::Unicode_UTF_8 => String::from_utf8(string_vec).ok(),
        TextEncoding::Unicode_UTF_16 =>  String::from_utf16(&to_u16_le(&string_vec).as_slice()).ok(),
        TextEncoding::Unicode_BigEndian => String::from_utf16(&to_u16_be(&string_vec).as_slice()).ok(),
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
