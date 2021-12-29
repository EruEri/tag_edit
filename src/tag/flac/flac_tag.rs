use std::{fs::OpenOptions, io::Read};

use crate::tag::traits::StringConvert;

use super::flac_metadata_block::FlacMetadataBlock;

pub (crate) const FLAC_ID: &'static str = "fLaC";



pub struct FlacTag {
    _id: String,
    stream_info: FlacMetadataBlock,
    metadata_blocks: Vec<FlacMetadataBlock>,
    music_data : Vec<u8>
}

impl FlacTag {
    pub fn from_path(path : &str) -> Option<Self> {
        let mut file = OpenOptions::new().read(true).open(path).ok()?;
        let mut buffer = vec![];
        let mut metadata_blocks = vec![];
        file.read_to_end(&mut buffer).ok()?;
        let flac_id = buffer.drain(0..4).collect::<Vec<u8>>().to_utf8()?;
        if flac_id != FLAC_ID { return  None; }
        let (stream_info, mut is_last) = FlacMetadataBlock::new(&mut buffer)?;
        while !is_last {
            let (block, last) = FlacMetadataBlock::new(&mut buffer)?;
            metadata_blocks.push(block);
            is_last = last;
        }
        Some(Self{
            _id: FLAC_ID.to_string(),
            stream_info,
            metadata_blocks,
            music_data : buffer
        })
    }
}

impl FlacTag {
    pub fn artist(&self) -> Option<String> {
        Some(
            self.metadata_blocks
                .iter()
                .filter_map(|flac_block  | {
                    match flac_block.as_vorbis_comments_frame() {
                        None => None,
                        Some(vorbis) => {
                           vorbis.get_artist()
                        }
                    }
                })
                .collect::<Vec<String>>()
                .join(", ")
            )
    }
}