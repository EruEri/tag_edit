use std::convert::TryInto;

use crate::util::{number::u24, traits::RawSize};

use super::flac_metadata_block_data::{FlacMetadataBlockData, VorbisCommentBlock, PictureBlock};

const LAST_BLOCK_FLAG : u8 = 0b10_000_000;
const BLOCK_TYPE_FLAG : u8 = !LAST_BLOCK_FLAG;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum FlacMetadataBlockType {
    STREAMINFO = 0,
    PADDING,
    APPLICATION,
    SEEKTABLE,
    VORBISCOMMENT,
    CUESHEET,
    PICTURE
}


impl FlacMetadataBlockType {
    pub(crate) fn from_raw_value(value : u8) -> Option<Self> {
        match value {
            0 => Some(Self::STREAMINFO),
            1 => Some(Self::PADDING),
            2 => Some(Self::APPLICATION),
            3 => Some(Self::SEEKTABLE),
            4 => Some(Self::VORBISCOMMENT),
            5 => Some(Self::CUESHEET),
            6 => Some(Self::PICTURE),
            _ => None
        }
    }
}

pub(crate) struct FlacMetadataBlock {
    is_last_block : bool,
    block_type : FlacMetadataBlockType,
    metadata_len : u24,
    data : FlacMetadataBlockData
}

impl RawSize for FlacMetadataBlock {
    fn raw_size(&self) -> usize {
        4 + self.data.raw_size()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push( (self.is_last_block as u8).rotate_right(1)  | (self.block_type as u8) );
        bytes.append(&mut self.metadata_len.to_be_bytes().to_vec());
        bytes.append(&mut self.data.raw_bytes());
        bytes
    }
}

impl FlacMetadataBlock {
    pub (crate) fn new(buffer : &mut Vec<u8>) -> Option<(Self, bool)> {
        let block_info = buffer.remove(0);
        let is_last_block = (block_info & LAST_BLOCK_FLAG) == LAST_BLOCK_FLAG;
        let block_type = FlacMetadataBlockType::from_raw_value(block_info & BLOCK_TYPE_FLAG)?;
        let len_bytes = buffer.drain(0..3).collect::<Vec<u8>>();
        let len = u24::from_be_bytes(len_bytes.try_into().unwrap());
        println!("block type : {:?}", block_type);
        println!("metadata len : {}", len.value());
        let data = FlacMetadataBlockData::new(buffer, len.value(), &block_type)?;
        Some((
            Self {
                is_last_block,
                block_type,
                metadata_len: len,
                data
            }
        , is_last_block))
    }

    pub (crate) fn default_from(block_type: FlacMetadataBlockType) -> Self {
        let data = FlacMetadataBlockData::default_from(&block_type);
        Self {
            is_last_block: false,
            block_type,
            metadata_len: u24::from(data.raw_size() as u32),
            data
        }
    }
    pub (crate) fn new_picture_block(picture_block : PictureBlock) -> Self {
        Self {
            is_last_block: false,
            block_type: FlacMetadataBlockType::PICTURE,
            metadata_len: (picture_block.raw_size() as u32).into(),
            data: FlacMetadataBlockData::PICTURE(picture_block),
        }
    }
    pub (crate) fn update_size(&mut self){ 
        self.metadata_len = (self.data.raw_size() as u32).into()
    }
}

impl FlacMetadataBlock {
    pub (crate) fn block_type(&self) -> &FlacMetadataBlockType {
        &self.block_type
    }
    pub (crate) fn set_last(&mut self, is_last: bool) {
        self.is_last_block = is_last
    }
    pub (crate) fn as_vorbis_comments_block(&self) -> Option<&VorbisCommentBlock> {
        self.data.as_vorbis_comments_block()
    }
    pub (crate) fn as_vorbis_comments_block_mut(&mut self) -> Option<&mut VorbisCommentBlock> {
        self.data.as_vorbis_comments_block_mut()
    }

    pub (crate) fn as_picture_block(&self) -> Option<&PictureBlock> {
        self.data.as_picture_block()
    }
}
