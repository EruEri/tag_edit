use crate::tag::id3::id3_frame_value::FrameValue::NoValue;
use std::convert::TryInto;
use std::str::FromStr;

use crate::tag::id3::id3_frameid::ID3FRAMEID;
use crate::tag::id3::id3_header_flag::{ID3FRAMEHEADERFLAGSB1, ID3FRAMEHEADERFLAGSB2};
use crate::tag::id3::id3_header_flag::ID3FRAMEHEADERFLAGSB1::*;
use crate::tag::id3::id3_header_flag::ID3FRAMEHEADERFLAGSB2::*;
use crate::tag::traits::{FrameSize, RawSize};

use super::id3_frame_value::{AttachedPictureFrame, CommentFrame, FrameValue, TextFrame, UnsyncLyricsFrame};

pub(crate) struct ID3FRAME {
    frame_id : ID3FRAMEID,
    /// Size of the frame : header include
    size : u32,
    _flag_byte_1 : Vec<ID3FRAMEHEADERFLAGSB1>,
    _flag_byte_2 : Vec<ID3FRAMEHEADERFLAGSB2>,
    value : FrameValue
}

impl From<(ID3FRAMEID, FrameValue)> for ID3FRAME {
    fn from((frame_id,value): (ID3FRAMEID, FrameValue)) -> Self {
        let _flag_byte_1 = vec![];
        let _flag_byte_2 = vec![];
        let size = (value.raw_size() + 10) as u32;
        Self {
            frame_id,
            size,
            _flag_byte_1,
            _flag_byte_2,
            value
        }
    }
}

impl ID3FRAME {

    
    pub(crate) fn new(buffer: &mut Vec<u8>) -> Option<Self>{
        println!("buffer lenght : {}", buffer.len());
        if buffer.len() <= 10 {
            return None;
        }
        let s = String::from_utf8(buffer.drain(0..4).collect()).unwrap();
        let frame_id = ID3FRAMEID::from_str(s.as_str()).ok()?;
        let size = u32::from_be_bytes(buffer.drain(0..4).collect::<Vec<u8>>().try_into().unwrap());
        if (size + 2) as usize >= buffer.len() { return None; }
        let mut _flag_byte_1 = vec![];
        let mut _flag_byte_2 = vec![];
        let flag1 = buffer.remove(0);
        let flag2 = buffer.remove(0);
        let frame_size = size + 10;
        println!("{} ->  size : {} ", frame_id, frame_size);
        if (flag1 & (FileAlterPreservation as u8) ) == (FileAlterPreservation as u8) {
            _flag_byte_1.push(FileAlterPreservation)
        }
        
        if (flag1 & (TagAlterPreservation as u8)) == (TagAlterPreservation as u8) {
            _flag_byte_1.push(TagAlterPreservation)
        }
        
        if (flag1 & (ReadOnly as u8)) == (ReadOnly as u8) {
            _flag_byte_1.push(ReadOnly)
        }
        
        if (flag2 & (Compression as u8) ) == Compression as u8  {
            _flag_byte_2.push(Compression)
        }
        if (flag2 & Encryption as u8)  == Encryption as u8  {
            _flag_byte_2.push(Encryption);
        }
        
        if (flag2 & (GroupingIdentity) as u8) == (GroupingIdentity as u8) {
            _flag_byte_2.push(GroupingIdentity);
        }
        let value = match FrameValue::new(buffer, frame_id, size){
            Some(f) => f,
            None => NoValue,
        };
        Some(Self {
            frame_id,
            size: frame_size,
            _flag_byte_1,
            _flag_byte_2,
            value
        })
    }

    pub (crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.frame_id as u8);
        bytes.append(&mut self.size.to_be_bytes().to_vec());
        let mut flag1 = 0;
        let mut flag2 = 0;
        for flag in self._flag_byte_1.iter(){
            flag1 |= *flag as u8
        }
        for flag in self._flag_byte_2.iter(){
            flag2 |= *flag as u8
        } 
        bytes.push(flag1);
        bytes.push(flag2);
        bytes.append(&mut self.value.raw_bytes());
        bytes
    }
}

impl FrameSize for ID3FRAME {
    fn total_size(&self) -> u32 {
        self.size
    }
}

impl ID3FRAME {

    pub(crate) fn recalcule_size(&mut self) {
        self.size = self.value.raw_size() as u32 + 10
    }

    pub (crate) fn get_frame_id(&self) -> &ID3FRAMEID {
        &self.frame_id
    }
    pub(crate) fn get_frame_value(&self) -> &FrameValue {
        &self.value
    }
    pub (crate) fn get_frame_value_mut(&mut self) -> &mut FrameValue {
        &mut self.value
    }
    pub(crate) fn as_attached_picture_frame(&self) -> Option<&AttachedPictureFrame> {
        self.value.as_attached_picture_frame()
    }

    pub(crate) fn as_attached_picture_frame_mut(&mut self) -> Option<&mut AttachedPictureFrame> {
        self.value.as_attached_picture_frame_mut()
    }

    pub (crate) fn as_unsynchroned_lyrics_frame(&self) -> Option<&UnsyncLyricsFrame> {
        self.value.as_unsynchroned_lyrics_frame()
    }
    pub (crate) fn as_unsynchroned_lyrics_frame_mut(&mut self) -> Option<&mut UnsyncLyricsFrame> {
        self.value.as_unsynchroned_lyrics_frame_mut()
    }

    pub (crate) fn as_comment_frame(&self) -> Option<&CommentFrame>{
        self.value.as_comment_frame()
    }
    pub (crate) fn as_comment_frame_mut(&mut self) -> Option<&mut CommentFrame>{
        self.value.as_comment_frame_mut()
    }
    pub(crate) fn as_text_frame(&self) -> Option<&TextFrame>{
        self.value.as_text_frame() 
    }
    pub(crate) fn as_text_frame_mut(&mut self) -> Option<&mut TextFrame>{
        self.value.as_text_frame_mut() 
    }
}