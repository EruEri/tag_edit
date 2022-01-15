use std::{fs::File, string::FromUtf8Error, io::Read, convert::TryInto};

use crate::id3::id3_header_flag::ID3HeaderFLAG;

use super::file_format::AudioFormat;
use super::file_format::AudioFormat::{FLAC, MP3, OTHER};

pub (crate) const LSBYTE_MASK : u16 = 0x00FF;

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
fn is_id3(s: &String) -> bool {
    s == "ID3"
}

fn is_flac(s: &String) -> bool{
    s == "fLaC"
}

pub (crate) fn read_type_audio_file(file: &mut File) -> Result<(AudioFormat, usize), FromUtf8Error> {
    let mut buffer = [0,0,0,0,0,0,0,0,0,0];
    let _ = file.read(&mut buffer);
    //let flac = String::from_utf8(buffer[0..4].into_vec());
    let id3 = String::from_utf8(buffer[0..3].to_vec())?;
    if is_id3(&id3) { 
        let flag = buffer[5];
        let unsync_flag = ID3HeaderFLAG::Unsynchronisation as u8;
        let size = if (flag & unsync_flag) != unsync_flag {
            //println!("To unsynchsafe");
            unsynchsafe(u32::from_be_bytes(buffer[6..].try_into().unwrap()))
        }else {
            //println!("unsynchsafe");
            u32::from_be_bytes(buffer[6..].try_into().unwrap())
        };
        return Ok( (MP3, size as usize)); 
    }
    if is_flac(&String::from_utf8(buffer[0..4].to_vec())?){ return Ok( (FLAC, 0) );  }
    Ok((OTHER, 0))
}



