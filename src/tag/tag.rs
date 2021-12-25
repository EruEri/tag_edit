
use crate::tag::id3::id3_tag::ID3TAG;
use super::{id3::id3_frameid::ID3FRAMEID::*, traits::TagSize};
pub enum Tag {
    ID3(ID3TAG)
}

impl Tag {
    pub(crate) fn as_bytes(&self) -> Vec<u8> {
        match self {
            Tag::ID3(tag) => tag.as_bytes(),
        }
    }

    pub(crate) fn get_size(&self) -> u32 {
        match self {
            Self::ID3(tag) => tag.size()
        }
    }
}

impl Tag {

    /// Retrieve all the pictures contained in the tag. An empty `Vec`
    /// if the tag contains 0 picture
    pub fn attached_pictures(&self) -> Vec<&Vec<u8>> {
        match self {
            Self::ID3(t) => t.get_attached_picture()
        }
    }
    pub fn artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag ) => tag.get_text_from_text_frame(&TPE1)
        }
    }
    pub fn set_artist(&mut self, name : String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TPE1, name),
        }
    }
    pub fn album_artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TPE2)
        }
    }
    pub(crate) fn set_album_artist(&mut self, artist: String) {
        match self {
            Self::ID3(tag ) => tag.set_text_frame(TPE2, artist)
        }
    }
    
    pub fn album(&self) -> Option<String> {
        match self {
            Self::ID3(t) => t.get_text_from_text_frame(&TALB)
        }
    }
    pub fn set_album(&mut self, album: String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TALB, album),
        }
    }
    pub fn genre(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TCON)
        }
    }
    pub(crate) fn set_genre(&mut self, genre: String) {
        match self {
            Tag::ID3(tag ) => tag.set_text_frame(TCON, genre)
        }
    }
    pub fn publisher(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TPUB)
        }
    }
    pub(crate) fn set_publisher(&mut self, publisher: String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TPUB, publisher),
        }
    }
    pub fn bpm(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TBPM)
        }
    }
    pub fn set_bpm(&mut self, bpm : u16){
        match self {
            Self::ID3(tag) => tag.set_text_frame(TBPM, bpm.to_string())
        }
    }
    pub fn composers(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TCOM),
        }
    }
    pub fn set_composers(&mut self, composers : String){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TCOM, composers)
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
    pub fn set_encoder(&mut self, encoder : String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TENC, encoder),
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
    pub fn set_title(&mut self, title : String){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TIT2, title),
        }
    }
    pub fn music_len(&self) -> Option<usize>{
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TLEN)?.parse().ok()
            
        }
    }
    pub fn year(&self) -> Option<i16>{
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TYER)?.parse().ok()
            
        }
    }
    pub fn set_year(&mut self, year : i16){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TYER, year.to_string())
        }
    } 
    pub fn track_position(&self) -> Option<String> {
        match self {
            Tag::ID3(tag ) => tag.get_text_from_text_frame(&TRCK)
        }
    }
    pub fn set_track_position(&mut self, track_pos: u16, out_of: Option<u16>) {
        match self {
            Tag::ID3(tag) => {
                let text = match out_of {
                    Some(n) => format!("{}/{}", track_pos, n),
                    None =>  track_pos.to_string()
                };
                tag.set_text_frame(TRCK, text)
            }
        }
    }
    pub fn album_part(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TPOS)
        }
    }
    pub fn set_album_part(&mut self, album_part: u16, out_of: Option<u16>) {
        match self {
            Tag::ID3(tag) => {
                let text = match out_of {
                    Some(n) => format!("{}/{}", album_part, n),
                    None =>  album_part.to_string()
                };
                tag.set_text_frame(TRCK, text)
            },
        }
    }
    pub fn lyrics(&self) -> Option<Vec<String>> {
        match self {
            Self::ID3(tag) => tag.get_unsynch_lyrics()
        }
    }
    pub fn comments(&self) -> Option<Vec<(String, String)>> {
        match self {
            Self::ID3(tag) => tag.get_comments()
        }
    }

}