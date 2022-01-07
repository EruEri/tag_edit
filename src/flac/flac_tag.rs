use std::io::{Error, Write};
use std::{fs::OpenOptions, io::Read};

use crate::id3::code::picture_code::picture_type::PictureType;
use crate::util::traits::{RawSize, StringConvert};

use super::flac_metadata_block::{FlacMetadataBlock, FlacMetadataBlockType};

use super::flac_metadata_block::FlacMetadataBlockType::*;
use super::flac_metadata_block_data::PictureBlock;

pub(crate) const FLAC_ID: &'static str = "fLaC";

pub struct FlacTag {
    filename : String,
    _id: String,
    stream_info: FlacMetadataBlock,
    metadata_blocks: Vec<FlacMetadataBlock>,
    music_data: Vec<u8>,
}

impl FlacTag {
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
    pub fn overwrite_flac(&self) -> Result<(), Error> {
        self.write_flac(self.filename.as_str())
    }
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
    pub fn title(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_title(),
            }
        })
    }

    pub fn artist(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_artist(),
            }
        })
    }

    pub fn album(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_album(),
            }
        })
    }

    pub fn album_artist(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_album_artist(),
            }
        })
    }

    pub fn genre(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_genre(),
            }
        })
    }
    pub fn copyright(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_copyright(),
            }
        })
    }
    pub fn date(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_date(),
            }
        })
    }
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
    pub fn comments(&self) -> Option<String> {
        self.metadata_blocks.iter().find_map(|flac_block| {
            match flac_block.as_vorbis_comments_block() {
                None => None,
                Some(vorbis) => vorbis.get_comments(),
            }
        })
    }

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

    pub fn pictures(&self) -> Vec<&Vec<u8>> {
        self.metadata_blocks
            .iter()
            .filter_map(|flac_block| match flac_block.as_picture_block() {
                None => None,
                Some(pc) => Some(pc.get_picture_data()),
            })
            .collect()
    }

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
