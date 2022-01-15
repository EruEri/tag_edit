//! Allow to create an ID3v2.3 tag from scratch
use std::{
    fs::{File, OpenOptions},
    io::{Error, Read, Write, Seek, SeekFrom},
};

use crate::util::{file_format::AudioFormat, function::read_type_audio_file};

use crate::id3::{
    code::picture_code::picture_type::PictureType,
    id3_frameid::{ID3FRAMEID, ID3TEXTFRAMEID, ID3TEXTFRAMEID::*},
    id3_tag::ID3TAG,
};

use crate::{
    util::file_format::PictureFormat,
    tag_error::TagError,
};


/// Builder for ID3v2.3 tag
pub struct ID3TagBuilder {
    id3_tag: ID3TAG,
}

impl ID3TagBuilder {
    /// Create the Tag builder without any frames in
    pub fn new() -> Self {
        Self {
            id3_tag: ID3TAG::new_empty_tag(),
        }
    }
    /// Add a text frame to the tag.
    /// Replace the content if the text frame already exists
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TagBuilder;
    /// use tag_edit::ID3TEXTFRAMEID;
    /// let mut tag_builder = ID3TagBuilder::new();
    /// tag_builder
    /// .add_text_frame(ID3TEXTFRAMEID::TIT2, "A title")
    /// .add_text_frame(ID3TEXTFRAMEID::TPE1, "A singer")
    /// .add_text_frame(ID3TEXTFRAMEID::TALB, "An album");
    ///
    ///
    /// ```
    pub fn add_text_frame(&mut self, frame_id: ID3TEXTFRAMEID, content: &str) -> &mut Self {
        self.id3_tag
            .set_text_frame(ID3FRAMEID::TEXTFRAME(frame_id), content.to_string());
        self
    }
    pub fn remove_text_frame(&mut self, frame_id: ID3TEXTFRAMEID) -> &mut Self {
        self.id3_tag.remove_frames(&ID3FRAMEID::TEXTFRAME(frame_id));
        self
    }
    /// Set the song artist (TPE1)
    ///
    pub fn set_artist(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TPE1, content)
    }
    /// Returns the album's artist (TPE2)
    ///
    pub fn set_album_artist(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TPE2, content)
    }
    /// Returns the album name (TABL)
    ///
    pub fn set_album(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TALB, content)
    }
    /// Set the genre (TCON)
    ///
    pub fn set_genre(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TCON, content)
    }
    /// Set the publisher (TPUB)
    ///
    pub fn set_publisher(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TPUB, content)
    }
    /// Set the beats per minutes of the song (TBPM)
    ///
    pub fn set_bpm(&mut self, content: u16) -> &mut Self {
        self.add_text_frame(TBPM, content.to_string().as_str())
    }
    /// Set the composers (TCOM)
    ///
    pub fn set_composers(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TCOM, content)
    }
    /// Set the encoder (TENC)
    ///
    pub fn set_encoder(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TENC, content)
    }

    /// Set the title (TIT2)
    ///
    pub fn set_title(&mut self, content: &str) -> &mut Self {
        self.add_text_frame(TIT2, content)
    }
    /// Set the track year (TYER)
    ///
    pub fn set_year(&mut self, content: i16) -> &mut Self {
        self.add_text_frame(TYER, content.to_string().as_str())
    }
    /// Set the track position in the album (TRCK)
    ///
    /// Arguments:
    /// * `track_pos` : track position
    /// * `out_of` : album's number of tracks
    ///
    ///
    pub fn set_track_position(&mut self, track_pos: u16, out_of: Option<u16>) -> &mut Self {
        let content = match out_of {
            Some(n) => format!("{}/{}", track_pos, n),
            None => track_pos.to_string(),
        };
        self.add_text_frame(TRCK, content.as_str())
    }
    /// Set the track's disc position (TPOS)
    ///
    /// Arguments:
    /// * `disc` : position of track's disc
    /// * `out_of` : album's number of discs
    ///
    pub fn set_disc(&mut self, disc: u16, out_of: Option<u16>) -> &mut Self {
        let content = match out_of {
            Some(n) => format!("{}/{}", disc, n),
            None => disc.to_string(),
        };
        self.add_text_frame(TPOS, content.as_str())
    }
    /// Add Unsynchronized lyrics to the tag
    ///
    /// # Errors
    /// This function will return an `TagError` if :
    ///
    /// * lang parameter is not ascii or length != 3
    /// * Tuple(`lang`, `description`) already exists in the lyrics frames
    ///
    ///
    pub fn add_lyrics(
        &mut self,
        lang: &str,
        description: Option<String>,
        text: String,
    ) -> Result<&mut Self, TagError> {
        match self.id3_tag.add_lyrics(lang, description, text) {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
    /// Add a comment to the tag
    ///
    /// # Errors
    /// This function will return an `TagError` if :
    ///
    /// * lang parameter is not ascii or length != 3
    /// * Tuple(`lang`, `description`) already exists in the comments frames
    ///
    ///
    pub fn add_comment(
        &mut self,
        lang: &str,
        description: Option<String>,
        text: String,
    ) -> Result<&mut Self, TagError> {
        match self
            .id3_tag
            .add_comment(lang, description, text)
        {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
    /// Add an image to the tag's attached pictures with pictures's raw bytes
    ///
    /// See the [ID3TagBuilder::add_picture_from_file] method to add an image from a file
    ///
    /// Arguments
    /// * `image_format` : (PNG | JPEG)
    /// * `picture_data` : pictures's raw bytes
    /// * `picture_type` :
    /// * `description`  : image short description
    ///
    pub fn add_picture(
        &mut self,
        picture_data: &Vec<u8>,
        image_format: PictureFormat,
        picture_type: Option<PictureType>,
        description: Option<String>,
    ) -> &mut Self {
        self.id3_tag
            .add_picture(&image_format, picture_data, picture_type, description);
        self
    }

    /// Add an image to the tag's attached pictures where the picture is in a file
    ///
    /// See the [ID3TagBuilder::add_picture] method to add an image with raw bytes
    /// Arguments
    /// * `file_path`    : path to picture
    /// * `image_format` : (PNG | JPEG)
    /// * `picture_type` :
    /// * `description`  : image short description
    ///
    pub fn add_picture_from_file(
        &mut self,
        file_path: &str,
        image_format: PictureFormat,
        picture_type: Option<PictureType>,
        description: Option<String>,
    ) -> Result<&mut Self, Error> {
        let mut image_buffer = vec![];
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut image_buffer)?;
        Ok(self.add_picture(&image_buffer, image_format, picture_type, description))
    }

    /// Write the tag and the audio content at `path`.
    /// The file will be created if doesn't exist or will be truncated if exists
    pub fn write_to(&self, path: &str) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        file.write_all(&mut self.id3_tag.as_bytes().as_slice())?;
        Ok(())
    }

    /// Replace a tag in a file with the tag created
    /// 
    /// # Errors
    /// 
    /// Returns [`TagError`]
    /// 
    /// If the tag found isn't an ID3v2 tag 
    /// 
    /// Or an [`std::io::Error`]
    /// 
    /// 
    pub fn replace_tag(&self, path: &str) -> Result<(), TagError> {
        let file_opt = OpenOptions::new()
            .read(true)
            .create(false)
            .truncate(false)
            .open(path);
        match file_opt {
            Err(e) => Err(TagError::IoError(e)),
            Ok(mut file) => {
                if let Ok((audio_type, size)) = read_type_audio_file(&mut file) {
                    let mut audio_content = vec![];
                    let _ = file.seek(SeekFrom::Start(size as u64));
                    let _ = file.read_to_end(&mut audio_content)?;
                    if audio_type != AudioFormat::MP3 { return Err(TagError::ID3TagNotFound) ;}
                    drop(file);
                    let mut file_bis = OpenOptions::new()
                    .create(false)
                    .write(true)
                    .truncate(true)
                    .open(path)?;
                    
                    let _ = file_bis.write(&mut self.id3_tag.as_bytes().as_slice())?;
                    let _ = file_bis.write(&mut audio_content)?;
                    Ok(())
                } else {
                    Err(TagError::ID3TagNotFound)
                }
            }
        }
    }
}
