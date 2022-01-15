use std::io::{Error, Write};
use std::{fs::OpenOptions, io::Read};

use crate::id3::code::picture_code::picture_type::PictureType;
use crate::util::traits::{RawSize, StringConvert};

use super::flac_metadata_block::{FlacMetadataBlock, FlacMetadataBlockType};

use super::flac_metadata_block::FlacMetadataBlockType::*;
use super::flac_metadata_block_data::PictureBlock;

pub(crate) const FLAC_ID: &'static str = "fLaC";

/// Metadata for Flac file
/// 
/// # Warning
/// All keys for the vorbis comments (custom keys or already existing keys ) are all set in uppercase
/// in the vorbis comments
/// 
/// ```
/// use tag_edit::FlacTag;
/// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
/// flactag.set_custom_field("CuSt_Key", "Value");
/// assert_eq!(flactag.get_custom_field("cust_key").unwrap(), "Value".to_string());
/// ```
/// 
pub struct FlacTag {
    filename : String,
    _id: String,
    stream_info: FlacMetadataBlock,
    metadata_blocks: Vec<FlacMetadataBlock>,
    music_data: Vec<u8>,
}

impl FlacTag {
    /// Create `FlacTag` from the path of a flac file
    ///
    pub fn from_path(path: &str) -> Option<Self> {
        let mut file = OpenOptions::new().read(true).open(path).ok()?;
        let mut buffer = vec![];
        let mut metadata_blocks = vec![];
        file.read_to_end(&mut buffer).ok()?;
        let flac_id = buffer.drain(0..4).collect::<Vec<u8>>().to_utf8()?;
        if flac_id != FLAC_ID {
            return None;
        }
        let (stream_info, mut is_last) = FlacMetadataBlock::new(&mut buffer)?;
        while !is_last {
            let (block, last) = FlacMetadataBlock::new(&mut buffer)?;
            metadata_blocks.push(block);
            is_last = last;
        }
        Some(Self {
            filename: path.into(),
            _id: FLAC_ID.to_string(),
            stream_info,
            metadata_blocks,
            music_data: buffer,
        })
    }

    pub(crate) fn insert_metadata_block(&mut self, mut block: FlacMetadataBlock) {
        if let Some(last_block) = self.metadata_blocks.last_mut() {
            if last_block.block_type() == &PADDING {
                block.set_last(false);
                let length = self.metadata_blocks.len();
                self.metadata_blocks.insert(length - 2, block);
            } else {
                last_block.set_last(false);
                block.set_last(true);
                self.metadata_blocks.push(block)
            }
        } else {
            block.set_last(true);
            self.metadata_blocks.push(block)
        }
    }
    /// Overwrite the flac origin file
    pub fn overwrite_flac(&self) -> Result<(), Error> {
        self.write_flac(self.filename.as_str())
    }
    /// Write the tag and the audio content at `path`.
    /// 
    /// The file will be created if doesn't exist or will be truncated if exists
    pub fn write_flac(&self, path: &str) -> Result<(), Error> {
        let mut output = OpenOptions::new()
        .create(true).read(false).write(true).truncate(true)
        .open(path)?;
        let _ = output.write(self.into_bytes().as_slice())?;
        Ok(())
    }

    pub(crate) fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self._id.clone().into_bytes());
        bytes.append(&mut self.stream_info.raw_bytes());
        self.metadata_blocks
            .iter()
            .for_each(|block| bytes.append(&mut block.raw_bytes()));
        bytes.append(&mut self.music_data.clone());
        bytes
    }

    pub(crate) fn get_block_mut(
        &mut self,
        block_type: &FlacMetadataBlockType,
    ) -> Option<&mut FlacMetadataBlock> {
        self.metadata_blocks
            .iter_mut()
            .find(|flac_block| flac_block.block_type() == block_type)
    }
}

// Getter
impl FlacTag {
    /// Returns the title of the song (key : "TITLE")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert_eq!(flactag.title().unwrap(), "Sleepless");
    /// 
    /// 
    /// ```
    pub fn title(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_title(),
            }
        })
    }

    /// Returns the artist's name (key : "ARTIST")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert_eq!(flactag.artist().unwrap(), "JUNNA");
    /// 
    /// 
    /// ```
    pub fn artist(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_artist(),
            }
        })
    }
    /// Returns the album (key : "ALBUM")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert_eq!(flactag.album().unwrap(), "20×20");
    /// 
    /// 
    /// ```
    pub fn album(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_album(),
            }
        })
    }

    /// Returns the artist's name of the album (key : "ALBUMARTIST")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert_eq!(flactag.album_artist().unwrap(), "JUNNA");
    /// 
    /// 
    /// ```
    pub fn album_artist(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_album_artist(),
            }
        })
    }

    /// Returns the genre of the song (key : "GENRE")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// if let Some(genre) = flactag.genre(){
    /// 
    /// }
    /// 
    /// 
    /// ```
    pub fn genre(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_genre(),
            }
        })
    }
    /// Retuns the copyright (key : COPYRIGHT)
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert_eq!(flactag.copyright().unwrap(), "(P)FlyingDog, Inc.")
    /// ```
    pub fn copyright(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_copyright(),
            }
        })
    }
    /// Returns the date (key : "GENRE")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// if let Some(date) = flactag.date(){
    /// 
    /// }
    /// 
    /// 
    /// ```
    pub fn date(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_date(),
            }
        })
    }
    /// Returns the composers (key : "COMPOSER")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert!(flactag.composer().is_some())
    /// 
    /// 
    /// ```
    pub fn composer(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_composer(),
            }
        })
    }

    pub fn disc(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_disc(),
            }
        })
    }
    pub fn total_disc(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.total_disc(),
            }
        })
    }

    pub fn track_position(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_track_position(),
            }
        })
    }
    pub fn total_track(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.total_track(),
            }
        })
    }
    /// Returns the comments (key : "COMMENT")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert!(flactag.comments().is_some())
    /// 
    /// 
    /// ```
    pub fn comments(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_comments(),
            }
        })
    }
    /// Returns the disc identifiant if exists
    pub fn disc_id(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_disc_id(),
            }
        })
    }

    pub fn organisation(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_organisation(),
            }
        })
    }
    /// Returns the pictures cintained in the file.
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// assert!(!flactag.pictures().is_empty())
    /// 
    /// 
    /// ```
    pub fn pictures(&self) -> Vec<&Vec<u8>> {
        self.metadata_blocks
            .iter()
            .filter_map(|flac_block| match flac_block.as_picture_block() {
                None => None,
                Some(pc) => Some(pc.get_picture_data()),
            })
            .collect()
    }
    /// Returns the field for an artitrary key.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// let custom_field = flactag.get_custom_field("A KEY");
    /// 
    /// 
    /// ```
    pub fn get_custom_field(&self, field : &str) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match  flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_custom_field(field)              
            }
        })
    }
}
// Setter
impl FlacTag {
    /// Set the title of the song (key : "TITLE")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_title("A title");
    /// assert_eq!(flactag.title().unwrap(), "A title");
    /// 
    /// 
    pub fn set_title(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_title(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_title(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }

    /// Replace the existing artists  (key : "ARTIST")
    /// 
    /// To add an artist to the medatada, see [FlacTag::add_artist]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_artist("An Artist");
    /// assert_eq!(flactag.artist().unwrap(), "An Artist");
    /// 
    /// 
    pub fn set_artist(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_artist(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_artist(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add an artist the existing artists  (key : "ARTIST")
    /// 
    /// To replace and set an artist to the medatada, see [FlacTag::set_artist]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_artist("artist1");
    /// flactag.add_artist("artist2");
    /// assert_eq!(flactag.artist().unwrap(), "artist1,artist2");
    /// 
    /// 
    pub fn add_artist(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_artist(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_artist(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the album of the song (key : "ALBUM")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_album("An Album");
    /// assert_eq!(flactag.album().unwrap(), "An Album");
    /// 
    /// 
    pub fn set_album(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_album(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_album(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the album's artist (key : "ALBUMARTIST")
    /// 
    /// To add other album artist to the medatada, see [FlacTag::add_album_artist]
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_album_artist("A main Artist");
    /// assert_eq!(flactag.album_artist().unwrap(), "A main Artist");
    /// 
    /// 
    pub fn set_album_artist(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_album_artist(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_album_artist(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add an album's artist to the existing artists  (key : "ALBUMARTIST")
    /// 
    /// To replace and set an album's artist to the medatada, see [FlacTag::set_album_artist]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_album_artist("artist1");
    /// flactag.add_album_artist("artist2");
    /// assert_eq!(flactag.album_artist().unwrap(), "artist1,artist2");
    /// 
    pub fn add_album_artist(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_album_artist(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_album_artist(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the genre (key : "GENRE")
    /// 
    /// To add other genre to the medatada, see [FlacTag::add_genre]
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_genre("A genre");
    /// assert_eq!(flactag.genre().unwrap(), "A genre");
    /// 
    /// 
    pub fn set_genre(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_genre(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_genre(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add a genre to the existing genre  (key : "GENRE")
    /// 
    /// To replace and set a genre to the medatada, see [FlacTag::set_genre]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_genre("genre1");
    /// flactag.add_genre("genre2");
    /// assert_eq!(flactag.genre().unwrap(), "genre1,genre2");
    /// 
    pub fn add_genre(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_genre(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_genre(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the copyright (key : "COPYRIGHT")
    pub fn set_copyright(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_copyright(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_copyright(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the date (key : "DATE")
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_date("2021");
    /// assert_eq!(flactag.date().unwrap(), "2021");
    /// 
    /// 
    pub fn set_date(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_date(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_date(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the composer (key : "COMPOSER")
    /// 
    /// To add other composers to the medatada, see [FlacTag::add_composer]
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_composer("A composer");
    /// assert_eq!(flactag.composer().unwrap(), "A composer");
    /// 
    /// 
    pub fn set_composer(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_composer(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_composer(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add a composer to the existing composers  (key : "COMPOSER")
    /// 
    /// To replace and set a composer to the medatada, see [FlacTag::set_composer]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_composer("composer1");
    /// flactag.add_composer("composer2");
    /// assert_eq!(flactag.composer().unwrap(), "composer1,composer2");
    /// 
    pub fn add_composer(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_composer(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_composer(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the BPM (key : "BPM")
    pub fn set_bpm(&mut self, content: u16) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_bpm(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_bpm(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the track disc of the song (key: "DISC")
    pub fn set_disc(&mut self, content: u16) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_disc(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_disc(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the number of disc contained in the album ("DISCTOTAL")
    pub fn set_total_disc(&mut self, content: u16) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_total_disc(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_total_disc(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the track position of the song in the disc (key : "TRACKPOSITION")
    pub fn set_track_position(&mut self, content: u16) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_track_position(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_track_position(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the number total of tracks contained in the disc
    pub fn set_total_track(&mut self, content: u16) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_total_track(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_total_track(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set a comment (key : "COMMENT")
    /// 
    /// To add other comment to the medatada, see [FlacTag::add_comment]
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_composer("A composer");
    /// assert_eq!(flactag.composer().unwrap(), "A composer");
    /// 
    /// 
    pub fn set_comment(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_comment(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_comment(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add a comment to the existing comments  (key : "COMMENT")
    /// 
    /// To replace and set a composer to the medatada, see [FlacTag::set_comment]
    /// 
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// flactag.set_comment("comment1");
    /// flactag.add_comment("comment2");
    /// assert_eq!(flactag.comment().unwrap(), "comment1,comment2");
    /// 
    pub fn add_comment(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_comment(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.add_comment(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    
    pub fn set_organisation(&mut self, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_organisation(content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_organisation(content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Set the field for an artitrary key.
    /// 
    /// # Example
    /// 
    /// ```
    /// use tag_edit::FlacTag;
    /// 
    /// let mut flactag = FlacTag::from_path("file_test/flac/03. Sleepless.flac").unwrap();
    /// let custom_field = flactag.set_custom_field("A KEY", "A Value");
    /// 
    /// 
    /// ```
    pub fn set_custom_field(&mut self, field : &str, content: &str) {
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_custom_field(field, content);
            flac_vorbis_block.update_size()
        } else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_custom_field(field, content);
            flac_vorbis_frame.update_size();
            self.insert_metadata_block(flac_vorbis_frame);
        }
    }
    /// Add a picture to the file
    /// 
    /// 
    /// See the [FlacTag::add_picture_from_path] method to add an image from a file
    /// 
    /// Arguments
    /// * `picture_type` : see [tag_edit::PictureType]
    /// * `mime_type` : jpeg | png | ... 
    /// * `description` : an optional description of the image
    /// * `picture_width`  : width of the picture 
    /// * `picture_height`  : heigth of the picture 
    /// * `color_depth`  : The color depth of the picture in bits-per-pixel
    /// * `number_color_used`  : For indexed-color pictures (e.g. GIF), the number of colors used, or 0 for non-indexed pictures.
    /// *` picuture_data` : The binary picture data
    /// 
    /// 
    pub fn add_picture(
        &mut self,
        picture_type: PictureType,
        mime_type: &str,
        description: Option<&str>,
        picture_width: u32,
        picture_height: u32,
        color_depth: u32,
        number_color_used: Option<u32>,
        picuture_data: &Vec<u8>,
    ) {
        let picture_block = PictureBlock::new(
            picture_type,
            mime_type,
            description,
            picture_width,
            picture_height,
            color_depth,
            number_color_used,
            picuture_data,
        );
        let flac_picture_block = FlacMetadataBlock::new_picture_block(picture_block);
        self.insert_metadata_block(flac_picture_block);
    }
    /// Add a picture to the file
    /// 
    /// 
    /// See the [FlacTag::add_picture] method to add an image from a binary picture data
    /// 
    /// Arguments
    /// *` picture_path` : image path
    /// * `picture_type` : see [tag_edit::PictureType]
    /// * `mime_type` : jpeg | png | ... 
    /// * `description` : an optional description of the image
    /// * `picture_width`  : width of the picture 
    /// * `picture_height`  : heigth of the picture 
    /// * `color_depth`  : The color depth of the picture in bits-per-pixel
    /// * `number_color_used`  : For indexed-color pictures (e.g. GIF), the number of colors used, or 0 for non-indexed pictures.
    /// 
    /// 
    pub fn add_picture_from_path(
        &mut self,
        picture_path : &str,
        picture_type: PictureType,
        mime_type: &str,
        description: Option<&str>,
        picture_width: u32,
        picture_height: u32,
        color_depth: u32,
        number_color_used: Option<u32>,
    ) -> Result<(), Error> {
        let mut data = vec![];
        let mut file = OpenOptions::new().create(false).read(true).open(picture_path)?;
        file.read_to_end(&mut data)?;
        Ok(
            self.add_picture(picture_type, mime_type, description, picture_width, picture_height, color_depth, number_color_used, &data)
        )
    }
}

impl FlacTag {
    pub fn remove_title(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_title();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_artist(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_artist();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_album_artist(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_album_artist();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_album(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_album();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_genre(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_genre();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_copyright(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_copyright();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_date(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_date();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_composer(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_composer();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_disc(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_disc();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_total_disc(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_total_disc();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_track_position(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_track_position();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_bpm(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_bpm();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_total_track(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_total_track();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_comments(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_comments();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_organisation(&mut self) {
        self.metadata_blocks
            .iter_mut()
            .find(|f| f.block_type() == &VORBISCOMMENT)
            .and_then(|flac_block| {
                flac_block
                    .as_vorbis_comments_block_mut()
                    .unwrap()
                    .remove_organisation();
                Some(flac_block)
            })
            .and_then(|flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_all_pictures(&mut self) {
        self.metadata_blocks
            .retain(|flac_block| flac_block.block_type() != &FlacMetadataBlockType::PICTURE)
    }
    pub fn remove_custom_field(&mut self, field : &str) /*-> Option<String>*/ {
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then(|flac_block| {
            let value = flac_block.as_vorbis_comments_block_mut()
            .unwrap()
            .remove_custom_field(field);
            Some((flac_block, value))
        })
        .and_then(|(f, value)| {
            f.update_size();
            Some(value)
        });
    }
}
