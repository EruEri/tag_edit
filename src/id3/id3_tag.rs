use std::fs::{File, OpenOptions};
use std::io::{Error, Read, Write};

use super::code::picture_code::picture_type::PictureType;
use super::id3_frame_value::{
    AttachedPictureFrame, CommentFrame, FrameValue, TextFrame, UnsyncLyricsFrame,
};
use super::id3_frameid::ID3FRAMEID;
use crate::id3::id3_frame::ID3FRAME;
use crate::id3::id3_frameid::ID3FRAMEID::*;
use crate::id3::id3_frameid::ID3TEXTFRAMEID::*;
use crate::id3::id3_header_flag::ID3HeaderFLAG;
use crate::id3::id3_header_flag::ID3HeaderFLAG::{
    ExperimentalIndicator, ExtendedHeader, Unsynchronisation,
};
use crate::tag_error::TagError;
use crate::util::file_format::PictureFormat;
use crate::util::function::{synchsafe, unsynchsafe};
use crate::util::traits::{FrameSize, TagSize};


/// Metadata for mp3 file
pub struct ID3TAG {
    file_path: String,
    _identifier: String,
    major_version: u8,
    _minor_version: u8,
    size: u32,
    _flags_header: Vec<ID3HeaderFLAG>,
    frames: Vec<ID3FRAME>,
    padding: i32,
    music_data : Vec<u8>
}

impl ID3TAG {

    pub fn from_path(path: &str) -> Option<Self> {
        let mut buffer = vec![];
        let mut file = OpenOptions::new().create(false).read(true).write(false).open(path).ok()?;
        file.read_to_end(&mut buffer).ok()?;
        Self::new(path, &mut buffer).ok()
    }
    pub(crate) fn new(file_path: &str, buffer: &mut Vec<u8>) -> Result<Self, ()> {
        if buffer.len() <= 10 {
            return Err(());
        }
        let id = String::from_utf8(buffer.drain(0..3).collect()).unwrap();
        let major_version = buffer.remove(0);
        if major_version != 3 {
            return Err(());
        }
        let _minor_version = buffer.remove(0);
        let mut _flags_header = vec![];
        let mut frames = vec![];
        let flag = buffer.remove(0);
        if (flag & (Unsynchronisation as u8)) == (Unsynchronisation as u8) {
            _flags_header.push(Unsynchronisation)
        };
        if (flag & (ExtendedHeader as u8)) == (ExtendedHeader as u8) {
            _flags_header.push(ExtendedHeader)
        };
        if (flag & (ExperimentalIndicator as u8)) == (ExperimentalIndicator as u8) {
            _flags_header.push(ExperimentalIndicator)
        };
        let buf = buffer.drain(0..4).collect::<Vec<u8>>();
        let size_from_buffer = u32::from_be_bytes([
            buf.get(0).unwrap().clone(),
            buf.get(1).unwrap().clone(),
            buf.get(2).unwrap().clone(),
            buf.get(3).unwrap().clone(),
        ]);
        let size = if !_flags_header.contains(&Unsynchronisation) {
            unsynchsafe(size_from_buffer)
        } else {
            size_from_buffer
        };
        let music_data = buffer.drain((size as usize - 10)..).collect();
        while buffer.len() > 10 {
            if let Some(frame) = ID3FRAME::new(buffer) {
                frames.push(frame);
                //println!()
            } else {
                //println!("No ID3FRAME");
                //println!("Padding size : {} ", &buffer.len());
                break;
            }
        }
        
        let padding = buffer.len() as i32 + 4;
        let mut tag = Self {
            file_path: file_path.into(),
            _identifier: id,
            major_version,
            _minor_version,
            size,
            _flags_header,
            frames,
            padding,
            music_data
        };
        tag.recalcule_all_size();
        Ok(tag)
    }

    pub(crate) fn new_empty_tag() -> Self {
        let padding_size = 10_000;
        Self {
            file_path: "".into(),
            _identifier: "ID3".into(),
            major_version: 3,
            _minor_version: 0,
            size: padding_size + 10,
            _flags_header: vec![],
            frames: vec![],
            padding: padding_size as i32,
            music_data: vec![]
        }
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
        bytes.append(&mut vec![0u8; self.padding as usize]);
        bytes.append(&mut self.music_data.clone());
        bytes
    }
}

impl TagSize for ID3TAG {
    fn size(&self) -> u32 {
        self.size
    }
}

impl ID3TAG {
    fn recalcule_all_size(&mut self) {
        self.frames
            .iter_mut()
            .for_each(|frame| frame.recalcule_size());
        self.recalcule_size()
    }

    pub fn recalcule_size(&mut self) {
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

    pub(crate) fn get_text_from_text_frame(&self, frame_id: &ID3FRAMEID) -> Option<String> {
        self.frames
            .iter()
            .find_map(|id3_frame| match id3_frame.as_text_frame() {
                None => None,
                Some(tf) => {
                    if id3_frame.get_frame_id() == frame_id {
                        Some(tf.get_text())
                    } else {
                        None
                    }
                }
            })
    }
    pub(crate) fn _get_text_frame_mut(&mut self, frame_id: &ID3FRAMEID) -> Option<&mut TextFrame> {
        self.frames
            .iter_mut()
            .find(|id3| id3.get_frame_id() == frame_id && frame_id.is_text_frame())?
            .as_text_frame_mut()
    }

    pub(crate) fn set_text_frame(&mut self, frame_id: ID3FRAMEID, text: String) {
        let major_version = self.major_version.clone();

        if let Some(frame) = self.get_frame_mut(&frame_id) {
            let text_frame = frame.as_text_frame_mut().unwrap();
            text_frame.set_text(text, major_version);
            frame.recalcule_size()
        } else {
            let value = TextFrame::new(major_version, text);
            let frame = (frame_id, FrameValue::TF(value)).into();
            self.frames.push(frame)
        }

        self.recalcule_size();
    }

    pub(crate) fn remove_frames(&mut self, frame_id: &ID3FRAMEID) {
        self.frames.retain(|frame| frame.get_frame_id() != frame_id);
        self.recalcule_size();
    }

    pub(crate) fn get_unsynch_lyrics(&self) -> Vec<String> {
        self.frames
            .iter()
            .filter_map(|id3_frame| match id3_frame.as_unsynchroned_lyrics_frame() {
                None => None,
                Some(f) => Some(f.get_lyrics().clone()),
            })
            .collect::<Vec<String>>()
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
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_all_lyrics();
    /// metadata.add_lyrics("eng", None, "Some Lyrics".into()).unwrap();
    /// assert_eq!(metadata.lyrics().first().unwrap(), &"Some Lyrics".to_string())
    ///
    /// ```
    pub fn add_lyrics(
        &mut self,
        lang: &str,
        description: Option<String>,
        text: String,
    ) -> Result<(), TagError> {
        let description = description.unwrap_or("".into());
        if lang.len() != 3 {
            Err(TagError::LangWrongSize)
        } else if self
            .frames
            .iter()
            .any(|frame| match frame.as_unsynchroned_lyrics_frame() {
                None => false,
                Some(f) => f.get_language() == &lang && f.get_content_description() == &text,
            })
        {
            Err(TagError::ReusedLangDescription)
        } else {
            let frame_value = UnsyncLyricsFrame::new(lang.into(), description, text);
            let frame = (ID3FRAMEID::USLT, FrameValue::ULF(frame_value)).into();
            self.frames.push(frame);
            self.recalcule_size();
            Ok(())
        }
    }

    pub(crate) fn get_comments(&self) -> Vec<(String, String)> {
        self.frames
            .iter()
            .filter_map(|id3_frame| match id3_frame.as_comment_frame() {
                None => None,
                Some(cf) => Some((cf.get_description().clone(), cf.get_text().clone())),
            })
            .collect::<Vec<(String, String)>>()
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
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_all_comments();
    /// metadata.add_comment("eng", None, "A random comment".into()).unwrap();
    /// assert_eq!(metadata.comments().first().unwrap().1, "A random comment")
    ///
    /// ```
    pub fn add_comment(
        &mut self,
        lang: &str,
        description: Option<String>,
        text: String,
    ) -> Result<(), TagError> {
        let description = description.unwrap_or("".into());
        if lang.len() != 3 {
            Err(TagError::LangWrongSize)
        } else if self
            .frames
            .iter()
            .any(|frame| match frame.as_comment_frame() {
                None => false,
                Some(f) => f.get_language() == &lang && f.get_description() == &description,
            })
        {
            Err(TagError::ReusedLangDescription)
        } else {
            let frame_value = CommentFrame::new(lang.into(), description, text);
            let frame = (ID3FRAMEID::USLT, FrameValue::CF(frame_value)).into();
            self.frames.push(frame);
            self.recalcule_size();
            Ok(())
        }
    }

    pub(crate) fn get_frame_mut(&mut self, frame_id: &ID3FRAMEID) -> Option<&mut ID3FRAME> {
        self.frames
            .iter_mut()
            .find(|id3| id3.get_frame_id() == frame_id)
    }
}

impl ID3TAG {

    /// Overwrite the tag in the origin file
    pub fn overwrite_tag(&self) -> Result<(), Error>{
        self.write_tag(self.file_path.as_str())
    }

    /// Write the tag and the audio content at `path`.
    /// The file will be created if doesn't exist or will be truncated if exists
    pub fn write_tag(&self, path : &str) -> Result<(), Error> {
        let mut file = OpenOptions::new()
        .create(true).read(false).write(true).truncate(true)
        .open(path)?;
        let _ = file.write(self.as_bytes().as_slice())?;
        Ok(())
    }
    
    /// Retrieves all the pictures contained in the tag. An empty `Vec`
    /// if the tag doesn'n contain any picture
    pub fn attached_pictures(&self) -> Vec<&Vec<u8>> {
        self.frames
            .iter()
            .filter_map(|id3_frame| match id3_frame.as_attached_picture_frame() {
                None => None,
                Some(apf) => Some(apf.get_picture_data()),
            })
            .collect()
    }
    /// Add an image to the tag's attached pictures with pictures's raw bytes
    ///
    /// See the [ID3TAG::add_picture_from_file] method to add an image from a file
    ///
    /// Arguments
    /// * `image_format` : 
    /// * `picture_data` : pictures's raw bytes
    /// * `picture_type` :
    /// * `description`  : image short description
    ///
    pub fn add_picture(
        &mut self,
        image_format: PictureFormat,
        picture_data: &Vec<u8>,
        picture_type: Option<PictureType>,
        description: Option<String>,
    ) {
        let apic_value =
            AttachedPictureFrame::new(&image_format, picture_data, picture_type, description);
        let frame = (ID3FRAMEID::APIC, FrameValue::APF(apic_value)).into();
        self.frames.push(frame);
        self.recalcule_size()
    }
    /// Add an image to the tag's attached pictures from the image file
    ///
    /// See the [ID3TAG::add_picture] method to add an image with raw bytes
    /// Arguments
    /// * `file_path`    : path to picture
    /// * `image_format` : 
    /// * `picture_type` :
    /// * `description`  : image short description
    ///
    pub fn add_picture_from_file(
        &mut self,
        file_path: &str,
        image_format: PictureFormat,
        picture_type: Option<PictureType>,
        description: Option<String>,
    ) -> Result<(), Error> {
        let mut image_buffer = vec![];
        let mut file = File::open(file_path)?;
        file.read_to_end(&mut image_buffer)?;
        Ok(self.add_picture(image_format, &image_buffer, picture_type, description))
    }
    /// Removes all the pictures contains in the tag
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert!(!metadata.attached_pictures().is_empty());
    /// metadata.remove_all_attached_pictures();
    /// assert!(metadata.attached_pictures().is_empty())
    /// ```
    pub fn remove_all_attached_pictures(&mut self) {
        self.remove_frames(&APIC)
    }
    /// Returns the song artist (TPE1)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.artist().unwrap(), "Maon Kurosaki");
    ///
    ///
    /// ```
    pub fn artist(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TPE1))
    }
    /// Set the song artist (TPE1)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_artist("Song performer");
    /// assert_eq!(metadata.artist().unwrap(), "Song performer");
    ///
    /// ```
    pub fn set_artist(&mut self, name: &str) {
        self.set_text_frame(TEXTFRAME(TPE1), name.into())
    }
    /// Revome the song artist (TPE1)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.artist().is_some());
    /// metadata.remove_artist();
    /// assert!(metadata.artist().is_none());
    ///
    /// ```
    pub fn remove_artist(&mut self) {
        self.remove_frames(&TEXTFRAME(TPE1))
    }
    /// Returns the album's artist (TPE2)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.album_artist().unwrap(), "\u{feff}黒崎真音\u{0}");
    ///
    /// ```
    pub fn album_artist(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TPE2))
    }
    /// Set the album's artist (TPE2)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_artist("Artist album");
    /// assert_eq!(metadata.artist().unwrap(), "Artist album");
    ///
    /// ```
    pub fn set_album_artist(&mut self, artist: &str) {
        self.set_text_frame(TEXTFRAME(TPE2), artist.into())
    }
    /// Remove the album artist (TPE2)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.album_artist().is_some());
    /// metadata.remove_album_artist();
    /// assert!(metadata.album_artist().is_none());
    ///
    ///
    /// ```
    pub fn remove_album_artist(&mut self) {
        self.remove_frames(&TEXTFRAME(TPE2))
    }
    /// Returns the album name (TABL)
    ///
    /// # Example
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.album().unwrap(), "Butterfly Effect".to_string())
    ///
    ///
    /// ```
    pub fn album(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TALB))
    }
    /// Set the album name (TALB)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_album("An album Name");
    /// assert_eq!(metadata.album().unwrap(), "An album Name");
    /// ```
    pub fn set_album(&mut self, album: &str) {
        self.set_text_frame(TEXTFRAME(TALB), album.into())
    }
    /// Remove the album  (TALB)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.album().is_some());
    /// metadata.remove_album();
    /// assert!(metadata.album().is_none());
    ///
    ///
    /// ```
    pub fn remove_album(&mut self) {
        self.remove_frames(&TEXTFRAME(TALB))
    }
    /// Returns the genre (TCON)
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.genre().unwrap(), "J-Pop".to_string());
    /// ```
    pub fn genre(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TCON))
    }
    /// Set the genre (TCON)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_genre("A Genre".into());
    /// assert_eq!(metadata.genre().unwrap(), "A Genre".to_string());
    /// ```
    pub fn set_genre(&mut self, genre: &str) {
        self.set_text_frame(TEXTFRAME(TCON), genre.into())
    }
    /// Remove the genre (TCON)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert!(metadata.genre().is_some());
    /// metadata.remove_genre();
    /// assert!(metadata.genre().is_none());
    ///
    ///
    /// ```
    pub fn remove_genre(&mut self) {
        self.remove_frames(&TEXTFRAME(TCON))
    }
    /// Returns the publisher (TPUB)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// let publisher = metadata.publisher();
    /// assert!(publisher.is_none());
    /// ```
    pub fn publisher(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TPUB))
    }
    /// Set the publisher (TPUB)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_publisher("Some Publisher");
    /// assert!(metadata.publisher().is_some());
    /// ```
    pub fn set_publisher(&mut self, publisher: &str) {
        self.set_text_frame(TEXTFRAME(TPUB), publisher.into())
    }
    /// Remove the publisher (TPUB)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_publisher();
    /// assert!(metadata.publisher().is_none());
    ///
    ///
    /// ```
    pub fn remove_publisher(&mut self) {
        self.remove_frames(&TEXTFRAME(TPUB))
    }
    /// Returns the beats per minutes of the song (TBPM)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// let bpm = metadata.bpm();
    /// assert!(bpm.is_none())
    ///
    ///
    /// ```
    pub fn bpm(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TBPM))
    }
    /// Set the beats per minutes of the song (TBPM)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_bpm(100);
    /// assert_eq!(metadata.bpm().unwrap().parse::<u16>().unwrap(), 100)
    ///
    ///
    /// ```
    pub fn set_bpm(&mut self, bpm: u16) {
        self.set_text_frame(TEXTFRAME(TBPM), bpm.to_string())
    }
    /// Remove the track's BPM (TBPM)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_bpm();
    /// assert!(metadata.bpm().is_none());
    ///
    ///
    /// ```
    pub fn remove_bpm(&mut self) {
        self.remove_frames(&TEXTFRAME(TBPM))
    }
    /// Returns the composers of the track (TCOM)
    pub fn composers(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TCOM))
    }
    /// Set the composers (TCOM)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_composers("A composers".into());
    /// assert_eq!(metadata.composers().unwrap(), "A composers".to_string());
    /// ```
    pub fn set_composers(&mut self, composers: &str) {
        self.set_text_frame(TEXTFRAME(TCOM), composers.into())
    }
    /// Remove the composers (TCOM)
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_composers("A composers".into());
    /// assert!(metadata.composers().is_some());
    /// metadata.remove_composers();
    /// assert!(metadata.composers().is_none())
    ///
    pub fn remove_composers(&mut self) {
        self.remove_frames(&TEXTFRAME(TCOM))
    }
    /// Returns the copyright message (TCOP)
    pub fn copyright(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TCOP))
    }
    /// Return the recoding's day in DDMM format (TDAT)
    pub fn date(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TDAT))
    }
    /// Remove the date (TDAT)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_date();
    /// assert!(metadata.date().is_none());
    ///
    ///
    /// ```
    pub fn remove_date(&mut self) {
        self.remove_frames(&TEXTFRAME(TDAT))
    }
    /// Returns track's encoder (TBPM)
    pub fn encoded_by(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TENC))
    }
    /// Set the encoder (TENC)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_encoder("An encoder");
    /// assert_eq!(metadata.encoded_by().unwrap(), "An encoder");
    ///
    /// ```
    pub fn set_encoder(&mut self, encoder: &str) {
        self.set_text_frame(TEXTFRAME(TENC), encoder.into())
    }
    /// Remove the encoder (TENC)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_encoder();
    /// assert!(metadata.encoded_by().is_none());
    ///
    ///
    /// ```
    pub fn remove_encoder(&mut self) {
        self.remove_frames(&TEXTFRAME(TENC))
    }

    pub fn file_type(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TFLT))
    }
    /// Returns the track's time recording in HHMM format (TIME)
    pub fn time(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TIME))
    }
    /// Remove the track's time recording (TIME)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_time();
    /// assert!(metadata.time().is_none());
    ///
    ///
    /// ```
    pub fn remove_time(&mut self) {
        self.remove_frames(&TEXTFRAME(TIME))
    }
    /// Returns the title (TIT2)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.title().unwrap(), "VANISHING POINT");
    ///
    ///
    /// ```
    pub fn title(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TIT2))
    }
    /// Set the title (TIT2)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_title("A title");
    /// assert_eq!(metadata.title().unwrap(), "A title");
    ///
    /// ```
    pub fn set_title(&mut self, title: &str) {
        self.set_text_frame(TEXTFRAME(TIT2), title.into())
    }
    /// Remove the title (TIT2)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_title();
    /// assert!(metadata.title().is_none());
    ///
    ///
    /// ```
    pub fn remove_title(&mut self) {
        self.remove_frames(&TEXTFRAME(TIT2))
    }
    /// Returns track's length in milliseconds (TLEN)
    pub fn music_len(&self) -> Option<usize> {
        self.get_text_from_text_frame(&TEXTFRAME(TLEN))?
            .parse()
            .ok()
    }
    /// Remove the music's length (TLEN)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_music_len();
    /// assert!(metadata.music_len().is_none());
    ///
    ///
    /// ```
    pub fn remove_music_len(&mut self) {
        self.remove_frames(&TEXTFRAME(TLEN))
    }
    /// Returns the track's year (TYER)
    ///
    /// # Example
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.year().unwrap(), 2011)
    ///
    ///
    /// ```
    pub fn year(&self) -> Option<i16> {
        self.get_text_from_text_frame(&TEXTFRAME(TYER))?
            .parse()
            .ok()
    }
    /// Set the track year (TYER)
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_year(2021);
    /// assert_eq!(metadata.year().unwrap(), 2021);
    ///
    /// ```
    pub fn set_year(&mut self, year: i16) {
        self.set_text_frame(TEXTFRAME(TYER), year.to_string())
    }
    /// Remove the year (TYER)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_year();
    /// assert!(metadata.year().is_none());
    ///
    ///
    /// ```
    pub fn remove_year(&mut self) {
        self.remove_frames(&TEXTFRAME(TYER))
    }
    /// Returns the track position in the disc (TRCK)
    ///
    /// # Example
    /// ```
    /// use tag_edit::ID3TAG;
    /// let metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// assert_eq!(metadata.track_position().unwrap(), "2".to_string())
    ///
    ///
    /// ```
    pub fn track_position(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TRCK))
    }
    /// Set the track position in the album (TRCK)
    ///
    /// Arguments:
    /// * `track_pos` : track position
    /// * `out_of` : album's number of tracks
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_track_position(1, None);
    /// assert_eq!(metadata.track_position().unwrap(), "1");
    /// metadata.set_track_position(1, Some(10));
    /// assert_eq!(metadata.track_position().unwrap(), "1/10");
    ///
    ///
    /// ```
    pub fn set_track_position(&mut self, track_pos: u16, out_of: Option<u16>) {
        let text = match out_of {
            Some(n) => format!("{}/{}", track_pos, n),
            None => track_pos.to_string(),
        };
        self.set_text_frame(TEXTFRAME(TRCK), text)
    }
    /// Remove the track position in the album (TRCK)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_track_position();
    /// assert!(metadata.track_position().is_none());
    ///
    ///
    /// ```
    pub fn remove_track_position(&mut self) {
        self.remove_frames(&TEXTFRAME(TRCK))
    }
    /// Returns the track's disc position (TPOS)
    pub fn disc(&self) -> Option<String> {
        self.get_text_from_text_frame(&TEXTFRAME(TPOS))
    }
    /// Set the track's disc position (TPOS)
    ///
    /// Arguments:
    /// * `disc` : position of track's disc
    /// * `out_of` : album's number of discs
    ///
    /// # Examples
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.set_disc(2, None);
    /// assert_eq!(metadata.disc().unwrap(), "2".to_string());
    /// metadata.set_disc(2, Some(20));
    /// assert_eq!(metadata.disc().unwrap(), "2/20".to_string());
    ///
    /// ```
    pub fn set_disc(&mut self, disc: u16, out_of: Option<u16>) {
        let text = match out_of {
            Some(n) => format!("{}/{}", disc, n),
            None => disc.to_string(),
        };
        self.set_text_frame(TEXTFRAME(TPOS), text)
    }
    /// Remove the track's disc position (TPOS)
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_disc();
    /// assert!(metadata.disc().is_none());
    ///
    ///
    /// ```
    pub fn remove_disc(&mut self) {
        self.remove_frames(&TEXTFRAME(TPOS))
    }
    /// Returns the unsynchronized lyrics in the tag
    pub fn lyrics(&self) -> Vec<String> {
        self.get_unsynch_lyrics()
    }
    /// Remove all the unsynchronized lyrics in the tag
    ///
    /// # Example
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_all_lyrics();
    /// assert!(metadata.lyrics().is_empty())
    ///
    /// ```
    pub fn remove_all_lyrics(&mut self) {
        self.remove_frames(&USLT)
    }
    /// Returns the comments in the tags
    pub fn comments(&self) -> Vec<(String, String)> {
        self.get_comments()
    }
    /// Remove all the comments in the tag
    ///
    /// # Example
    /// ```
    /// use tag_edit::ID3TAG;
    /// let mut metadata = ID3TAG::from_path("file_test/mp3/02 VANISHING POINT.mp3").unwrap();
    /// metadata.remove_all_comments();
    /// assert!(metadata.comments().is_empty())
    ///
    ///
    /// ```
    pub fn remove_all_comments(&mut self) {
        self.remove_frames(&COMM)
    }
}
