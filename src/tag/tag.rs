
use crate::tag::id3::id3_tag::ID3TAG;
use super::id3::id3_frameid::ID3FRAMEID::*;
pub enum Tag {
    ID3(ID3TAG)
}

impl Tag {

    
    pub fn test(&self) {
        let _test = match self {
            Tag::ID3(tag) => {
                tag.get_text_from_text_frame(&TOWN)
            }
        }; 
    }
    pub fn attached_pictures(&self) -> Option<Vec<&Vec<u8>>> {
        match self {
            Self::ID3(t) => t.get_attached_picture()
        }
    }
    pub fn artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag ) => tag.get_text_from_text_frame(&TPE1)
        }
    }
    pub fn album_artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TPE2)
        }
    }
    
    pub fn album(&self) -> Option<String> {
        match self {
            Self::ID3(t) => t.get_text_from_text_frame(&TALB)
        }
    }
    pub fn genre(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TCON)
        }
    }
    pub fn publisher(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TPUB)
        }
    }
    pub fn bpm(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TBPM)
        }
    }
    pub fn composers(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TCOM),
        }
    }
    pub fn copyright(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TCOP),
        }
    }
    pub fn date(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TDAT),
        }
    }
    pub fn encoded_by(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TENC),
        }
    }
    pub fn file_type(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TFLT),
        }
    }
    pub fn time(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TIME)
        }
    }
    pub fn title(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TIT2),
        }
    }
    pub fn music_len(&self) -> Option<usize>{
        match self {
            Tag::ID3(tag) => {
                    tag.get_text_from_text_frame(&TLEN)?.parse().ok()
            }
        }
    }
    pub fn year(&self) -> Option<i16>{
        match self {
            Tag::ID3(tag) => {
                    tag.get_text_from_text_frame(&TYER)?.parse().ok()
            }
        }
    }
    pub fn track_position(&self) -> Option<String> {
        match self {
            Tag::ID3(tag ) => tag.get_text_from_text_frame(&TRCK)
        }
    }
    pub fn album_part(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TPOS)
        }
    }
    pub fn lyrics(&self) -> Option<Vec<String>> {
        match self {
            Self::ID3(tag) => tag.get_unsynch_lyrics()
        }
    }

}