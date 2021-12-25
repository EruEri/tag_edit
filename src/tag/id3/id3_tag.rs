use crate::tag::id3::id3_frame::ID3FRAME;
use crate::tag::id3::id3_header_flag::ID3HeaderFLAG;
use crate::tag::id3::id3_header_flag::ID3HeaderFLAG::{ExperimentalIndicator, ExtendedHeader, Unsynchronisation};
use crate::tag::traits::{FrameSize, TagSize};
use crate::util::function::{unsynchsafe, synchsafe};
use super::id3_frame_value::{FrameValue, TextFrame};
use super::id3_frameid::ID3FRAMEID;

pub struct ID3TAG {
    _identifier : String,
    major_version : u8,
    _minor_version : u8,
    size : u32,
    _flags_header : Vec<ID3HeaderFLAG>,
    frames : Vec<ID3FRAME>,
    padding : i32
}


impl ID3TAG {
    pub (crate) fn new(buffer : &mut Vec<u8>) -> Result<Self, ()> {
        if buffer.len() <= 10 { return Err(());}
        let id = String::from_utf8(buffer.drain(0..3).collect()).unwrap() ;
        let major_version = buffer.remove(0);
        if major_version != 3 {
            return  Err(());
        }
        let _minor_version = buffer.remove(0);
        let mut _flags_header = vec![];
        let mut frames = vec![];
        let flag = buffer.remove(0);
        if (flag & (Unsynchronisation as u8) ) == (Unsynchronisation as u8) { _flags_header.push(Unsynchronisation)};
        if (flag & (ExtendedHeader as u8)) == (ExtendedHeader as u8) { _flags_header.push(ExtendedHeader)};
        if (flag & (ExperimentalIndicator as u8)) == (ExperimentalIndicator as u8) { _flags_header.push(ExperimentalIndicator)};
        let buf  = buffer.drain(0..4).collect::<Vec<u8>>();
        let size_from_buffer = u32::from_be_bytes(
            [buf.get(0).unwrap().clone(), buf.get(1).unwrap().clone(), buf.get(2).unwrap().clone(), buf.get(3).unwrap().clone()]);
        let size = if !_flags_header.contains(&Unsynchronisation) { unsynchsafe(size_from_buffer)} else { size_from_buffer };
        while buffer.len() > 10 {
            if let Some(frame) = ID3FRAME::new(buffer){
                frames.push(frame);
                //println!()
            }else {
                //println!("No ID3FRAME");
                //println!("Padding size : {} ", &buffer.len());
                break;
            }
        }
        let mut tag = Self {
            _identifier: id,
            major_version,
            _minor_version,
            size,
            _flags_header,
            frames,
            padding : (buffer.len() as i32 + 4)
        };
        tag.recalcule_all_size();
        Ok (tag)
    }

    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.append(&mut self._identifier.clone().into_bytes());
        bytes.push(self.major_version);
        bytes.push(self._minor_version);
        let mut flags = 0;
        for flag in self._flags_header.iter() {
            flags |= *flag as u8
        }
        bytes.push(flags);
        let mut sync = synchsafe(self.size).to_be_bytes().to_vec();
        bytes.append(&mut sync);

        self.frames
        .iter()
        .for_each(|frame| bytes.append(&mut frame.as_bytes()));
        bytes.append(&mut  vec![0u8;self.padding as usize]);
        bytes
    }

}

impl TagSize for ID3TAG {
    fn size(&self) -> u32 {
        self.size
    }
}

impl ID3TAG {

    fn recalcule_all_size(&mut self){
        self
        .frames
        .iter_mut()
        .for_each(|frame| frame.recalcule_size());
        self.recalcule_size()
    }

    pub fn recalcule_size(&mut self){
        //self.size = self.frame_total_size() + (self.padding as u32)
        self.size = self.total_size()
    }

    pub fn total_size(&self) -> u32 {
        self.frame_total_size() + (self.padding as u32) + 10
    }
    
    pub fn frame_total_size(&self) -> u32 {
        let mut size = 0u32;
        for frame in self.frames.iter() {
            size += frame.total_size()
        }
        size
    }

    pub (crate) fn get_text_from_text_frame(&self, frame_id : &ID3FRAMEID) -> Option<String>{
        self
        .frames
        .iter()
        .find_map(|id3_frame| {
            match id3_frame.as_text_frame() {
                None => None,
                Some(tf) => if id3_frame.get_frame_id() == frame_id { Some(tf.get_text()) } else {None}
            }
        })
    }
    pub (crate) fn _get_text_frame_mut(&mut self, frame_id: &ID3FRAMEID) -> Option<&mut TextFrame> {
        self
        .frames
        .iter_mut()
        .find(|id3| id3.get_frame_id() == frame_id && frame_id.is_text_frame())?
        .as_text_frame_mut()
    }

    pub (crate) fn set_text_frame(&mut self, frame_id: ID3FRAMEID, text: String) {
        let major_version = self.major_version.clone();

        if let Some(frame) = self.get_frame_mut(&frame_id){
            let text_frame = frame.as_text_frame_mut().unwrap();
            text_frame.set_text(text, major_version);
            frame.recalcule_size()
        }else {
            let value = TextFrame::new(major_version, text);
            let frame = (frame_id, FrameValue::TF(value)).into();
            self.frames.push(frame)
        }

        self.recalcule_size();
    }

    pub(crate) fn remove_text_frame(&mut self, frame_id: &ID3FRAMEID) {
        self.frames
        .retain(|frame| frame.get_frame_id() != frame_id)
    }

    pub (crate) fn get_unsynch_lyrics(&self)-> Option<Vec<String>> {
        Some (self.frames.iter()
        .filter_map(|id3_frame| {
            match id3_frame.as_unsynchroned_lyrics_frame(){
                None => None,
                Some(f) => Some( f.get_lyrics().clone() ),
                
            }
        })
        .collect::<Vec<String>>()
     )
    }

    pub (crate) fn get_comments(&self) -> Option<Vec<(String, String)>> {
        Some(
            self.frames
            .iter()
            .filter_map(|id3_frame| {
                match id3_frame.as_comment_frame() {
                    None => None,
                    Some(cf) => Some((cf.get_description().clone(), cf.get_text().clone())),
                }
            })
            .collect::<Vec<(String, String)>>()
        )
    }

 
    pub(crate) fn get_frame_mut(&mut self, frame_id : &ID3FRAMEID) -> Option<&mut ID3FRAME> {
        self
        .frames
        .iter_mut()
        .find(|id3| {
            id3.get_frame_id() == frame_id
        } )
        
    }

    pub fn get_attached_picture(&self) -> Vec<&Vec<u8> > {
            self.frames.iter()
            .filter_map(|id3_frame| {
                match id3_frame.as_attached_picture_frame() {
                    None => None,
                    Some(apf) => Some( apf.get_picture_data())  
                }
            })
            .collect()
    }
}