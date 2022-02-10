//! A library to read and write tag from audio files.
//! Currently, only ID3v2.3 and Flac supported
//! 
//! 
//! 
//! 
//! Read tand write tag from mp3 file
//! 
//! 
//! 
//! ```no_run
//! use tag_edit::ID3TAG;
//! 
//! 
//! let mut metadata = ID3TAG::from_path("file_test/mp3/1-01 Dark seeks light.mp3").unwrap();
//! if let Some(_artist) = metadata.artist(){
//!     // do something
//! }
//! if let Some(_year) = metadata.year(){
//!     // do somthing else
//! }
//! 
//! metadata.set_bpm(100);
//! 
//! metadata.set_album("An album".into());
//! 
//! let _ = metadata.overwrite_tag();
//! 
//! 
//! 
//! ```
//! 
//! 
//! Create a tag and replace an existing tag
//! 
//! ```
//! use tag_edit::ID3TagBuilder;
//! use tag_edit::ID3TEXTFRAMEID;
//! 
//! let mut tag_builder = ID3TagBuilder::new();
//! tag_builder
//! .set_artist("An artist")
//! .set_album("An album")
//! .add_text_frame(ID3TEXTFRAMEID::TIT2, "A title")
//! .replace_tag("file_path");
//! 
//! 
//! 
//! ```
//! 
//! Read and write metadata from flac file
//! 
//! ```
//! use tag_edit::FlacTag;
//! let mut flac_tag = FlacTag::from_path("file_test/flac/02. believe in myself.flac").unwrap();
//! if let Some(_artist) = flac_tag.artist(){
//!     // do something
//! }
//! 
//! if let Some(_album) = flac_tag.album(){
//!     // do something else
//! }
//! 
//! flac_tag.set_disc(1);
//! 
//! 
//! let _ = flac_tag.overwrite_flac();
//! 
//! ```
//! 
//! 

pub use crate::id3_tag_builder::ID3TagBuilder;
pub use crate::tag_error::TagError;
pub use crate::id3::id3_frameid::ID3TEXTFRAMEID;
pub use crate::util::file_format::PictureFormat;
pub use crate::flac::flac_tag::FlacTag;
pub use crate::id3::code::picture_code::picture_type::PictureType;
pub use crate::id3::id3_tag::ID3TAG;


pub (crate) mod id3_tag_builder;
pub (crate) mod id3;
pub (crate) mod tag_error;
pub (crate) mod util;
pub (crate) mod flac;




#[cfg(test)]
#[allow(unused)]
mod test {
    const INPUT_FILE : &'static str = "file_test/mp3/02 VANISHING POINT.mp3";
    const OUTPUT_TEST : &'static str = "file_test/output/o.mp3";
    const IMAGE_PATH : &'static str = "file_test/image/mysfloreg.jpeg";
    const FLAC_FILE : &'static str = "file_test/flac/03. Sleepless.flac";
    const OUTPUT_F_TEST : &'static str = "file_test/output/f.flac";
    
    use std::{io::{Error, Read, Write}, fs::OpenOptions, collections::HashMap};

    use crate::{id3_tag_builder::ID3TagBuilder, FlacTag, ID3TAG};
    use crate::PictureType;
    use crate::PictureType::*;

    #[test]
    fn tag_builder() -> Result<(), Error>{
        let mut buff_data = vec![];
        let mut source = OpenOptions::new().create(false).read(true).open("file_test/mp3/01 Setsuna no Kajitsu.mp3")?;
        let _ = source.read_to_end(&mut buff_data)?;
        let mut out = OpenOptions::new().create(true).truncate(true).write(true).open(OUTPUT_TEST)?;
        out.write(&mut buff_data)?;
        let result = ID3TagBuilder::new()
        .set_title("刹那の果実")
        .set_artist("黒崎真音")
        .set_album("Mystical Flower")
        .add_text_frame(crate::ID3TEXTFRAMEID::TYER, "2015")
        .set_disc(1, Some(1))
        .set_track_position(2, Some(15))
        .add_picture_from_file(IMAGE_PATH,  crate::PictureFormat::JPEG, None, None).unwrap()
        .replace_tag(OUTPUT_TEST);
        if let Err(e) = result {
            panic!("an error occured : {:?}", e)
        }
        let metadata = ID3TAG::from_path(OUTPUT_TEST).unwrap();
        assert_eq!(metadata.album(), Some("Mystical Flower".to_string()));
        assert_eq!(metadata.year(), Some(2015));
        assert_eq!(metadata.disc(), Some("1/1".to_string()));
        Ok(())
    }
    #[test]
    fn mp3tag() -> Result<(), Error>{
        let mut dark_seek_light = ID3TAG::from_path("file_test/mp3/1-01 Dark seeks light.mp3").unwrap();
        dark_seek_light.set_album("An album of Yui ninomiya");
        dark_seek_light.set_title("Tesla Note Opening");
        dark_seek_light.set_artist("Yui ninomiya");
        dark_seek_light.write_tag("file_test/output/id3tag.mp3")
  
    }

    #[test]
    fn flac_read() -> Result<(), Error>{
        if let Some(mut flactag) = FlacTag::from_path(FLAC_FILE) {
            //flactag.set_title("Darwin game opening");
            //flactag.set_album("Tokyo 1/3650");
            //flactag.set_artist("Nanjo Yoshino");
            //flactag.add_artist("An other artist");
            //flactag.set_album_artist("Yohsino Nanjo");
            flactag.add_genre("An Junne Genre");
            flactag.set_album("An Junna Album : 20x20");
            flactag.remove_all_pictures();
            flactag.add_picture_from_path("file_test/image/2020.jpeg", CoverFront, crate::PictureFormat::JPEG, None, 1000, 867, 24, None)?;
            flactag.write_flac("file_test/output/testim.flac");
            Ok(())
            //assert_eq!(flactag.title(), Some("CHAIN".to_string()))
        }else {
            panic!("Not created")
        }
    }
    
}