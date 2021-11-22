use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::string::FromUtf8Error;
use crate::tag::audio_type::AudioType;
use crate::tag::audio_type::AudioType::{FLAC, MP3, OTHER};
use crate::tag::id3::id3_tag::ID3TAG;
use crate::tag::tag::Tag;
//use crate::util::function::synchsafe;
use super::tag::id3::id3_header_flag::ID3HeaderFLAG;
use super::util::function::unsynchsafe;


pub struct Metadata{
    file : File,
    file_type : AudioType,
    tag : Tag
}

fn is_id3(s: &String) -> bool {
    s == "ID3"
}

fn is_flac(s: &String) -> bool{
    s == "fLaC"
}

fn read_type_audio_file(file: &mut File) -> Result<(AudioType, usize), FromUtf8Error> {
    let mut buffer = [0,0,0,0,0,0,0,0,0,0];
    let _ = file.read(&mut buffer);
    //let flac = String::from_utf8(buffer[0..4].into_vec());
    let id3 = String::from_utf8(buffer[0..3].to_vec())?;
    if is_id3(&id3) { 
        let flag = buffer[5];
        let unsync_flag = ID3HeaderFLAG::Unsynchronisation as u8;
        let size = if (flag & unsync_flag) != unsync_flag {
            println!("To unsynchsafe");
            unsynchsafe(u32::from_be_bytes(buffer[6..].try_into().unwrap()))
        }else {
            println!("unsynchsafe");
            u32::from_be_bytes(buffer[6..].try_into().unwrap())
        };
        // println!("header size : {}", u32::from_be_bytes(buffer[6..].try_into().unwrap()));
        // println!("header size synchsafe : {}", synchsafe(u32::from_be_bytes(buffer[6..].try_into().unwrap())));
        // println!("header size unsynchsafe : {}", unsynchsafe(u32::from_be_bytes(buffer[6..].try_into().unwrap())));
        return Ok( (MP3, size as usize)); 
    }
    if is_flac(&String::from_utf8(buffer[0..4].to_vec())?){ return Ok( (FLAC, 0) );  }
    Ok((OTHER, 0))
}

impl Metadata {
    pub fn new(file_path: &str) -> Option<Self>{
        let mut file = OpenOptions::new().create(false).read(true).write(true).open(file_path).ok()?;
        let (audio_type, size) = read_type_audio_file(&mut file).ok()?;
        match audio_type {
            FLAC => todo!("FLac to implement"),
            MP3 => {
                let mut buffer = vec![0u8; size];
                let _ = file.seek(SeekFrom::Start(0));
                let _ = file.read_exact(&mut buffer);
                let tag = ID3TAG::new(&mut buffer).ok()?;
                Some( Metadata {
                    file,
                    file_type: MP3,
                    tag : Tag::ID3(tag)
                }   )
            }
            OTHER => todo!("Other not implemented"),
        }
    }
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    pub fn attached_pictures(&self) -> Option<Vec<&Vec<u8>>> {
        self.tag.attached_pictures()
    }
    pub fn artist(&self) -> Option<String>{
        self.tag.artist()
    }
    pub fn album_artist(&self) -> Option<String> {
        self.tag.album_artist()
    }
    pub fn album(&self) -> Option<String>{
        self.tag.album()
    }
    pub fn genre(&self) -> Option<String> {
        self.tag.genre()
    }
    pub fn publisher(&self) -> Option<String> {
        self.tag.publisher()
    }
    pub fn bpm(&self) -> Option<String> {
        self.tag.bpm()
    }
    pub fn copyright(&self) -> Option<String> {
        self.tag.copyright()
    }
    pub fn date(&self) -> Option<String> {
        self.tag.date()
    }
    pub fn encoded_by(&self) -> Option<String> {
        self.tag.encoded_by()
    }
    pub fn file_type(&self) -> Option<String> {
        self.tag.file_type()   
    }
    pub fn time(&self) -> Option<String> {
        self.tag.time()
    }
    pub fn title(&self) -> Option<String> {
        self.tag.title()
    }
    pub fn music_len(&self) -> Option<usize> {
        self.tag.music_len()
    }
    pub fn year(&self) -> Option<i16> {
        self.tag.year()
    }
    pub fn track_position(&self) -> Option<String> {
        self.tag.track_position()
    }
    pub fn album_part(&self) -> Option<String> {
        self.tag.album_part()
    }

}