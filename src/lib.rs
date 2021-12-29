//! A library to read and write tag from audio files.
//! Currently, only ID3v2.3 supported
//! 
//! 
//! 
//! 
//! Read tag from file
//! 
//! 
//! 
//! ```no_run
//! use tag_edit::Metadata;
//! 
//! 
//! let mut metadata = Metadata::from_path("file_test/mp3/1-01 Dark seeks light.mp3").unwrap();
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
//! 

pub use crate::id3_tag_builder::ID3TagBuilder;
pub use crate::metadata::Metadata;
pub use crate::tag_error::TagError;
pub use crate::tag::id3::id3_frameid::ID3TEXTFRAMEID;
pub use crate::tag::file_format::PictureFormat;
pub use crate::tag::flac::flac_tag::FlacTag;


pub (crate) mod tag;
pub (crate) mod id3_tag_builder;
pub (crate) mod tag_error;
pub (crate) mod metadata;
pub (crate) mod util;




#[cfg(test)]
mod test {
    const INPUT_FILE : &'static str = "file_test/mp3/02 VANISHING POINT.mp3";
    const OUTPUT_TEST : &'static str = "file_test/output/o.mp3";
    const IMAGE_PATH : &'static str = "file_test/image/mysfloreg.jpeg";
    const FLAC_FILE : &'static str = "file_test/flac/02. CHAIN.flac";
    
    use std::{io::{Error, Read, Write}, fs::OpenOptions};

    use crate::{metadata::Metadata, id3_tag_builder::ID3TagBuilder, FlacTag};
    

    #[test]
    fn it_work(){
        assert_eq!(true, !false);
    }

    // #[test]
    // fn check_size_integrity(){
    //     if let Some(mut metadata) = Metadata::new(INPUT_FILE){
    //         //metadata.set_artist("Foo".into());
    //         //metadata.set_bpm(97);
    //         //metadata.set_publisher("BAR".into());
    //         assert_eq!(metadata.tag().get_size() as usize, metadata.tag().as_bytes().len())
    //     }else {
    //         panic!("Cannot create metadata")
    //     }
    // }
    #[test]
    fn comment_frame_test(){
        if let Some(metadata) = Metadata::from_path(INPUT_FILE){
            //metadata.set_artist("Foo".into());
            //metadata.set_bpm(97);
            //metadata.set_publisher("BAR".into());
            assert_eq!(metadata.artist(), Some("Maon Kurosaki".to_string()));
            //assert_eq!(metadata.tag().get_size() as usize, metadata.tag().as_bytes().len())
        }else {
            panic!("Cannot create metadata")
        }
    }

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
        let metadata = Metadata::from_path(OUTPUT_TEST).unwrap();
        assert_eq!(metadata.album(), Some("Mystical Flower".to_string()));
        assert_eq!(metadata.year(), Some(2015));
        assert_eq!(metadata.disc(), Some("1/1".to_string()));
        Ok(())
    }

    #[test]
    fn flac_read(){
        if let Some(flactag) = FlacTag::from_path(FLAC_FILE) {
            assert!(flactag.artist().is_some());
        }else {
            panic!("Not created")
        }
    }
    
}