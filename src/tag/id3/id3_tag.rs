use crate::tag::id3::id3_frame::ID3FRAME;
use crate::tag::id3::id3_header_flag::ID3HeaderFLAG;
use crate::tag::id3::id3_header_flag::ID3HeaderFLAG::{ExperimentalIndicator, ExtendedHeader, Unsynchronisation};
use crate::tag::traits::{FrameSize, TagSize};
use crate::util::function::unsynchsafe;
use super::id3_frameid::ID3FRAMEID;

pub struct ID3TAG {
    identifier : String,
    major_version : u8,
    minor_version : u8,
    size : u32,
    flags_header : Vec<ID3HeaderFLAG>,
    frames : Vec<ID3FRAME>,
    padding : u32
}


impl ID3TAG {
    pub (crate) fn new(buffer : &mut Vec<u8>) -> Result<Self, ()> {
        if buffer.len() <= 10 { return Err(());}
        let id = String::from_utf8(buffer.drain(0..3).collect()).unwrap() ;
        let major_version = buffer.remove(0);
        let minor_version = buffer.remove(0);
        let mut flags_header = vec![];
        let mut frames = vec![];
        let flag = buffer.remove(0);
        if (flag & (Unsynchronisation as u8) ) == (Unsynchronisation as u8) { flags_header.push(Unsynchronisation)};
        if (flag & (ExtendedHeader as u8)) == (ExtendedHeader as u8) { flags_header.push(ExtendedHeader)};
        if (flag & (ExperimentalIndicator as u8)) == (ExperimentalIndicator as u8) { flags_header.push(ExperimentalIndicator)};
        let buf  = buffer.drain(0..4).collect::<Vec<u8>>();
        let size_from_buffer = u32::from_be_bytes(
            [buf.get(0).unwrap().clone(), buf.get(1).unwrap().clone(), buf.get(2).unwrap().clone(), buf.get(3).unwrap().clone()]);
        let size = if !flags_header.contains(&Unsynchronisation) { unsynchsafe(size_from_buffer)} else { size_from_buffer };
        while buffer.len() > 10 {
            if let Some(frame) = ID3FRAME::new(buffer){
                frames.push(frame);
                println!()
            }else {
                println!("No ID3FRAME");
                println!("Padding size : {} ", &buffer.len());
                break;
            }
        }
        Ok (Self {
            identifier: id,
            major_version,
            minor_version,
            size,
            flags_header,
            frames,
            padding : (buffer.len() as u32 + 4)
        })
    }
}

impl TagSize for ID3TAG {
    fn size(&self) -> u32 {
        self.size
    }
}

impl ID3TAG {

    pub fn total_size(&self) -> u32 {
        self.frame_total_size() + self.padding + 10
    }
    
    pub fn frame_total_size(&self) -> u32 {
        let mut size = 0u32;
        for frame in self.frames.iter() {
            size += frame.total_size()
        }
        size
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

    pub (crate) fn get_comments(&self) -> Option<Vec<String>> {
        Some(
            self.frames
            .iter()
            .filter_map(|id3_frame| {
                match id3_frame.as_comment_frame() {
                    None => None,
                    Some(cf) => Some(cf.get_text().clone()),
                }
            })
            .collect::<Vec<String>>()
        )
    }

    pub (crate) fn get_text_from_text_frame(&self, frame_id : &ID3FRAMEID) -> Option<String>{
        if !frame_id.is_text_frame() {
            None
        }else {
          let mut string = self.frames.iter()
        .filter_map(|id3_frame| { 
            match id3_frame.as_text_frame(){
                Some(tf) => if &id3_frame.get_frame_id() == frame_id { Some(tf.get_text())} else {None},
                None => None,
            }
        })
        .collect::<Vec<String>>();
        if string.len() == 0 { 
            None 
        } else { Some(string.remove(0)) }

        }
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


    // pub fn get_attached_picture(&self) -> Option<Vec<&Vec<u8>>> {
    //     let pictures = self.frames.iter().filter(
    //         |id3_frame| id3_frame.get_frame_id() == ID3FRAMEID::APIC
    //           ).collect::<Vec<&ID3FRAME>>();
    //     if pictures.is_empty() {
    //         None
    //     }else {
            
    //         Some(
    //             pictures.into_iter()
    //             .map(|frame| frame.as_attached_picture_frame().unwrap().get_picture_data())
    //             .collect()
    //         )
    //     }
    // }
}