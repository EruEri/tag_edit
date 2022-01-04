use std::{fs::OpenOptions, io::Read};

use crate::util::traits::StringConvert;

use super::flac_metadata_block::{FlacMetadataBlock, FlacMetadataBlockType};

use super::flac_metadata_block::FlacMetadataBlockType::*;

pub(crate) const FLAC_ID: &'static str = "fLaC";

pub struct FlacTag {
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
            _id: FLAC_ID.to_string(),
            stream_info,
            metadata_blocks,
            music_data: buffer,
        })
    }

    pub(crate) fn insert_metadata_block(&mut self, mut block: FlacMetadataBlock){
        if let Some(last_block) = self.metadata_blocks.last_mut(){
            last_block.set_last(false);
            block.set_last(true);
            self.metadata_blocks.push(block)
        }else {
            block.set_last(true);
            self.metadata_blocks.push(block)
        }
        
    }

    pub (crate) fn get_block_mut(&mut self, block_type : &FlacMetadataBlockType) -> Option<&mut FlacMetadataBlock> {
        self.metadata_blocks
        .iter_mut()
        .find(|flac_block | {
            flac_block.block_type() == block_type
        })
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
}
impl FlacTag {
    pub fn set_title(&mut self, content : &str){
        if let Some(flac_vorbis_block) = self.get_block_mut(&VORBISCOMMENT) {
            let vorbis = flac_vorbis_block.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_title(content);
            flac_vorbis_block.update_size()
        }else {
            let mut flac_vorbis_frame = FlacMetadataBlock::default_from(VORBISCOMMENT);
            let vorbis = flac_vorbis_frame.as_vorbis_comments_block_mut().unwrap();
            vorbis.set_title(content);
            self.insert_metadata_block(flac_vorbis_frame);
        }
    } 
}

impl FlacTag {
    pub fn remove_title(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_title();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_artist(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_artist();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_album_artist(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_album_artist();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_album(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_album();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }

    pub fn remove_genre(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_genre();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_copyright(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_copyright();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_date(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_date();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_composer(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_composer();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_disc(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_disc();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_total_disc(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().total_disc();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_track_position(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_track_position();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_total_track(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_total_track();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_comments(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_comments();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }
    pub fn remove_organisation(&mut self){
        self.metadata_blocks
        .iter_mut()
        .find(|f| f.block_type() == &VORBISCOMMENT)
        .and_then( |flac_block| {
            flac_block.as_vorbis_comments_block_mut().unwrap().remove_organisation();
            Some(flac_block)
        })
        .and_then( |flac_block| Some(flac_block.update_size()));
    }



}