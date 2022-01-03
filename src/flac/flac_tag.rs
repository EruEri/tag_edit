use std::{fs::OpenOptions, io::Read};

use crate::util::traits::StringConvert;

use super::flac_metadata_block::FlacMetadataBlock;

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
}

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
