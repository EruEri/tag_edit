
use crate::tag::id3::id3_tag::ID3TAG;
use super::file_format::PictureFormat;
use super::id3::code::picture_code::picture_type::PictureType;
use super::traits::TagSize;
use crate::tag::id3::id3_frameid::ID3FRAMEID::*;
use crate::tag::id3::id3_frameid::ID3TEXTFRAMEID::*;

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

    pub fn attached_pictures(&self) -> Vec<&Vec<u8>> {
        match self {
            Self::ID3(t) => t.get_attached_picture()
        }
    }
    pub(crate) fn add_picture(&mut self, image_format: &PictureFormat, picture_data: &Vec<u8>, picture_type: Option<PictureType>, description: Option<String>) {
        match self {
            Self::ID3(tag) => tag.add_picture(image_format, picture_data, picture_type, description)
        }
    }
    pub fn remove_all_attached_pictures(&mut self){
        match self {
            Self::ID3(tag) => tag.remove_frames(&APIC)
        }
    }
    pub fn artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag ) => tag.get_text_from_text_frame(&TEXTFRAME(TPE1))
        }
    }
    pub fn set_artist(&mut self, name : String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TPE1), name),
        }
    }

    pub fn remove_artist(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TPE1)),
        }
    }
    
    pub fn album_artist(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TPE2))
        }
    }
    pub(crate) fn set_album_artist(&mut self, artist: String) {
        match self {
            Self::ID3(tag ) => tag.set_text_frame(TEXTFRAME(TPE2), artist)
        }
    }

    pub fn remove_album_artist(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TPE2)),
        }
    }
    
    pub fn album(&self) -> Option<String> {
        match self {
            Self::ID3(t) => t.get_text_from_text_frame(&TEXTFRAME(TALB))
        }
    }
    pub fn set_album(&mut self, album: String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TALB), album),
        }
    }

    pub fn remove_album(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TALB)),
        }
    }
    
    pub fn genre(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TCON))
        }
    }
    pub(crate) fn set_genre(&mut self, genre: String) {
        match self {
            Tag::ID3(tag ) => tag.set_text_frame(TEXTFRAME(TCON), genre)
        }
    }

    pub fn remove_genre(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TCON)),
        }
    }
    pub fn publisher(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TPUB))
        }
    }
    pub(crate) fn set_publisher(&mut self, publisher: String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TPUB), publisher),
        }
    }
    pub fn remove_publisher(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TPUB)),
        }
    }
    pub fn bpm(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TBPM))
        }
    }
    pub fn set_bpm(&mut self, bpm : u16){
        match self {
            Self::ID3(tag) => tag.set_text_frame(TEXTFRAME(TBPM), bpm.to_string())
        }
    }

    pub fn remove_bpm(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TBPM)),
        }
    }
    pub fn composers(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TCOM)),
        }
    }
    pub fn set_composers(&mut self, composers : &Vec<String>){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TCOM), composers.join(","))
        }
    }
    pub fn remove_composers(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TCOM)),
        }
    }
    pub fn copyright(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TCOP)),
        }
    }
    pub fn date(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TDAT)),
        }
    }

    pub fn remove_date(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TDAT)),
        }
    }
    pub fn encoded_by(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TENC)),
        }
    }
    pub fn set_encoder(&mut self, encoder : String) {
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TENC), encoder),
        }
    }
    pub fn remove_encoder(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TENC)),
        }
    }
    pub fn file_type(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TFLT)),
        }
    }
    pub fn time(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TIME))
        }
    }
    pub fn remove_time(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TIME)),
        }
    }
    pub fn title(&self) -> Option<String> {
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TIT2)),
        }
    }
    pub fn set_title(&mut self, title : String){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TIT2), title),
        }
    }
    pub fn remove_title(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TIT2)),
        }
    }
    pub fn music_len(&self) -> Option<usize>{
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TLEN))?.parse().ok()
            
        }
    }
    pub fn remove_music_len(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TLEN)),
        }
    }
    pub fn year(&self) -> Option<i16>{
        match self {
            Tag::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TYER))?.parse().ok()
            
        }
    }
    pub fn set_year(&mut self, year : i16){
        match self {
            Tag::ID3(tag) => tag.set_text_frame(TEXTFRAME(TYER), year.to_string())
        }
    }
    pub fn remove_year(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TYER)),
        }
    }
    pub fn track_position(&self) -> Option<String> {
        match self {
            Tag::ID3(tag ) => tag.get_text_from_text_frame(&TEXTFRAME(TRCK))
        }
    }
    pub fn set_track_position(&mut self, track_pos: u16, out_of: Option<u16>) {
        match self {
            Tag::ID3(tag) => {
                let text = match out_of {
                    Some(n) => format!("{}/{}", track_pos, n),
                    None =>  track_pos.to_string()
                };
                tag.set_text_frame(TEXTFRAME(TRCK), text)
            }
        }
    }
    pub fn remove_track_position(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TRCK)),
        }
    }
    pub fn disc(&self) -> Option<String> {
        match self {
            Self::ID3(tag) => tag.get_text_from_text_frame(&TEXTFRAME(TPOS))
        }
    }
    pub fn set_disc(&mut self, disc: u16, out_of: Option<u16>) {
        match self {
            Tag::ID3(tag) => {
                let text = match out_of {
                    Some(n) => format!("{}/{}", disc, n),
                    None =>  disc.to_string()
                };
                tag.set_text_frame(TEXTFRAME(TPOS), text)
            },
        }
    }
    pub fn remove_disc(&mut self) {
        match self {
            Tag::ID3(tag) => tag.remove_frames(&TEXTFRAME(TPOS)),
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