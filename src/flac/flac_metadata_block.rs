use std::convert::TryInto;

use crate::util::number::u24;

use super::flac_metadata_block_data::{FlacMetadataBlockData, VorbisCommentBlock};

const LAST_BLOCK_FLAG : u8 = 0b10_000_000;
const BLOCK_TYPE_FLAG : u8 = !LAST_BLOCK_FLAG;

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

impl FlacMetadataBlock {
    pub (crate) fn new(buffer : &mut Vec<u8>) -> Option<(Self, bool)> {
        let block_info = buffer.remove(0);
        let is_last_block = (block_info & LAST_BLOCK_FLAG) == LAST_BLOCK_FLAG;
        let block_type = FlacMetadataBlockType::from_raw_value(block_info & BLOCK_TYPE_FLAG)?;
        let len_bytes = buffer.drain(0..3).collect::<Vec<u8>>();
        let len = u24::from_be_bytes(len_bytes.try_into().unwrap());
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
}

impl FlacMetadataBlock {
    pub (crate) fn as_vorbis_comments_frame(&self) -> Option<&VorbisCommentBlock> {
        self.data.as_vorbis_comments_frame()
    }
}
