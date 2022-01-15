use crate::{
    id3::code::picture_code::picture_type::PictureType,
    util::{traits::{StringConvert, ToU16, ToU32}, vorbis_vector::VorbisVector},
    util::{number::u24, traits::RawSize},
};
use std::{convert::TryInto, fmt::Display, str::FromStr};

use super::flac_metadata_block::FlacMetadataBlockType;

const COMPACT_DISC_FLAG: u8 = 0b10_000_000;

const TRACK_TYPE_MASK: u8 = 0b10_000_000;
const PRE_EMPHASIS_MASK: u8 = 0b01_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ApplicationID {
    /// FlacFile
    ATCH,
    BSOL,
    BUGS,
    Cues,
    Fica,
    Ftol,
    MOTB,
    MPSE,
    MuML,
    RIFF,
    SFFL,
    SONY,
    SQEZ,
    TtWv,
    UITS,
    Aiff,
    Imag,
    Peem,
    Qfst,
    Riff,
    Tune,
    Xbat,
    Xmcd,
}

impl FromStr for ApplicationID {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ATCH" => Ok(Self::ATCH),
            "BSOL" => Ok(Self::BSOL),
            "BUGS" => Ok(Self::BUGS),
            "Cues" => Ok(Self::Cues),
            "Fica" => Ok(Self::Fica),
            "Ftol" => Ok(Self::Ftol),
            "MOTB" => Ok(Self::MOTB),
            "MPSE" => Ok(Self::MPSE),
            "MuML" => Ok(Self::MuML),
            "RIFF" => Ok(Self::RIFF),
            "SFFL" => Ok(Self::SFFL),
            "SONY" => Ok(Self::SONY),
            "SQEZ" => Ok(Self::SQEZ),
            "TtWv" => Ok(Self::TtWv),
            "UITS" => Ok(Self::UITS),
            "aiff" => Ok(Self::Aiff),
            "imag" => Ok(Self::Imag),
            "peem" => Ok(Self::Peem),
            "qfst" => Ok(Self::Qfst),
            "riff" => Ok(Self::Riff),
            "tune" => Ok(Self::Tune),
            "xbat" => Ok(Self::Xbat),
            "xmcd" => Ok(Self::Xmcd),
            _ => Err(()),
        }
    }
}
impl Display for ApplicationID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::ATCH => "ATCH",
            Self::BSOL => "BSOL",
            Self::BUGS => "BUGS",
            Self::Cues => "Cues",
            Self::Fica => "Fica",
            Self::Ftol => "Ftol",
            Self::MOTB => "MOTB",
            Self::MPSE => "MPSE",
            Self::MuML => "MuML",
            Self::RIFF => "RIFF",
            Self::SFFL => "SFFL",
            Self::SONY => "SONY",
            Self::SQEZ => "SQEZ",
            Self::TtWv => "TtWv",
            Self::UITS => "UITS",
            Self::Aiff => "aiff",
            Self::Imag => "imag",
            Self::Peem => "peem",
            Self::Qfst => "qfst",
            Self::Riff => "riff",
            Self::Tune => "tune",
            Self::Xbat => "xbat",
            Self::Xmcd => "xmcd",
        };
        write!(f, "{}", s)
    }
}

pub(crate) struct CueSheetTrackIndex {
    offset: u64,
    index_point_number: u8,
}

impl RawSize for CueSheetTrackIndex {
    fn raw_size(&self) -> usize {
        8 + 1 + 3
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.offset.to_be_bytes().to_vec());
        bytes.push(self.index_point_number);
        bytes.push(0);
        bytes.push(0);
        bytes.push(0);
        bytes
    }
}

impl CueSheetTrackIndex {
    pub(crate) fn new(buffer: &mut Vec<u8>) -> Option<Self> {
        let offset = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let index_point_number = buffer.remove(0);
        Some(Self {
            offset,
            index_point_number,
        })
    }
}

pub(crate) struct CueSheetTrack {
    track_offset: u64,
    track_number: u8, // not 0
    track_isrc: Vec<u8>,
    is_audio: bool,
    is_pre_emphasis: bool, // 13 null bytes
    tracks_index: Vec<CueSheetTrackIndex>,
}

impl RawSize for CueSheetTrack {
    fn raw_size(&self) -> usize {
        let mut track_index_len = 0;
        self.tracks_index
            .iter()
            .for_each(|csti| track_index_len += csti.raw_size());
        8 + 1 + 12 + 14 + 1 + track_index_len
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.track_offset.to_be_bytes().to_vec());
        bytes.push(self.track_number);
        bytes.append(&mut self.track_isrc.clone());
        let mut audio_pre_emphasis_byte = 0;
        if !self.is_audio {
            audio_pre_emphasis_byte |= TRACK_TYPE_MASK
        };
        if self.is_pre_emphasis {
            audio_pre_emphasis_byte |= PRE_EMPHASIS_MASK
        };
        bytes.push(audio_pre_emphasis_byte);
        bytes.append(&mut vec![0u8; 13]);
        bytes.push(self.tracks_index.len() as u8);
        self.tracks_index
            .iter()
            .for_each(|ctsi| bytes.append(&mut ctsi.raw_bytes()));
        bytes
    }
}

impl CueSheetTrack {
    pub(crate) fn new(buffer: &mut Vec<u8>) -> Option<Self> {
        let track_offset =
            u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let track_number = buffer.remove(0);
        let track_isrc = buffer.drain(0..12).collect::<Vec<u8>>();
        let audio_pre_emphisis = buffer.remove(0);
        let is_audio = (audio_pre_emphisis & TRACK_TYPE_MASK) != TRACK_TYPE_MASK;
        let is_pre_emphasis = (audio_pre_emphisis & PRE_EMPHASIS_MASK) == PRE_EMPHASIS_MASK;
        buffer.drain(0..13);
        let number_track_index_point = buffer.remove(0);
        let mut tracks_index = vec![];
        for _ in 0..number_track_index_point {
            let cue_sheet_index = CueSheetTrackIndex::new(buffer)?;
            tracks_index.push(cue_sheet_index);
        }
        Some(Self {
            track_offset,
            track_number,
            track_isrc,
            is_audio,
            is_pre_emphasis,
            tracks_index,
        })
    }
}

pub(crate) struct CueSheetBlock {
    media_catalog_number: String, // 128 bytes length
    number_lead_sample: u64,
    is_compact_disc: bool, // 1 + 258 bytes
    cuesheets_tracks: Vec<CueSheetTrack>,
}

impl RawSize for CueSheetBlock {
    fn raw_size(&self) -> usize {
        128 + 8 + 1 + 258 + 1 + self.cuesheets_tracks.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.media_catalog_number.clone().into_bytes());
        bytes.append(&mut self.number_lead_sample.to_be_bytes().to_vec());
        if self.is_compact_disc {
            bytes.push(COMPACT_DISC_FLAG)
        } else {
            bytes.push(0)
        };
        bytes.append(&mut vec![0u8; 258]);
        bytes.push(self.cuesheets_tracks.len() as u8);
        self.cuesheets_tracks
            .iter()
            .for_each(|cst| bytes.append(&mut cst.raw_bytes()));
        bytes
    }
}

pub(crate) struct StreamInfoBlock {
    min_block_size: u16,
    max_block_size: u16,
    min_frame_size: u24,
    max_frame_size: u24,
    sample_rate_nb_chan_bit_sample_nb_sample: u64,
    md5_signature: u128,
}

impl RawSize for StreamInfoBlock {
    fn raw_size(&self) -> usize {
        2 + 2 + 3 + 3 + 8 + 16
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.min_block_size.to_be_bytes().to_vec());
        bytes.append(&mut self.max_block_size.to_be_bytes().to_vec());
        bytes.append(&mut self.min_frame_size.to_be_bytes().to_vec());
        bytes.append(&mut self.max_frame_size.to_be_bytes().to_vec());
        bytes.append(
            &mut self
                .sample_rate_nb_chan_bit_sample_nb_sample
                .to_be_bytes()
                .to_vec(),
        );
        bytes.append(&mut self.md5_signature.to_be_bytes().to_vec());
        bytes
    }
}

pub(crate) struct PaddingBlock {
    nb_bytes: usize,
}
impl Default for PaddingBlock {
    fn default() -> Self {
        Self { nb_bytes: 0 }
    }
}
impl RawSize for PaddingBlock {
    fn raw_size(&self) -> usize {
        self.nb_bytes
    }

    fn raw_bytes(&self) -> Vec<u8> {
        vec![0u8; self.nb_bytes]
    }
}

pub(crate) struct ApplicationBlock {
    app_id: ApplicationID,
    data: Vec<u8>,
}

impl RawSize for ApplicationBlock {
    fn raw_size(&self) -> usize {
        4 + self.data.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut buffer = vec![];
        buffer.append(&mut self.app_id.to_string().into_bytes());
        buffer.append(&mut self.data.clone());
        buffer
    }
}
pub struct SeekPoint {
    sample_number_of_first_sample: u64,
    offset: u64,
    number_of_sample: u16,
}

impl RawSize for SeekPoint {
    fn raw_size(&self) -> usize {
        8 + 8 + 2
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.sample_number_of_first_sample.to_be_bytes().to_vec());
        bytes.append(&mut self.offset.to_be_bytes().to_vec());
        bytes.append(&mut self.number_of_sample.to_be_bytes().to_vec());
        bytes
    }
}

impl SeekPoint {
    fn new(buffer: &mut Vec<u8>) -> Option<Self> {
        let sample_number_of_first_sample =
            u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let offset = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let number_of_sample = buffer.drain(0..2).collect::<Vec<u8>>().u16_from_be()?;
        Some(Self {
            sample_number_of_first_sample,
            offset,
            number_of_sample,
        })
    }
}

pub(crate) struct SeekTableBlock {
    seek_points: Vec<SeekPoint>,
}

impl RawSize for SeekTableBlock {
    fn raw_size(&self) -> usize {
        18 * self.seek_points.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        self.seek_points
            .iter()
            .for_each(|seek_point| bytes.append(&mut seek_point.raw_bytes()));
        bytes
    }
}

impl SeekTableBlock {
    fn new(buffer: &mut Vec<u8>, nb_to_create: u32) -> Option<Self> {
        let mut seek_points = vec![];
        for _ in 0..nb_to_create {
            let seek_point = SeekPoint::new(buffer)?;
            seek_points.push(seek_point);
        }
        Some(Self { seek_points })
    }
}

pub(crate) struct VorbisCommentBlock {
    vendor_name: String,
    comments: VorbisVector
}
impl Default for VorbisCommentBlock {
    fn default() -> Self {
        Self {
            vendor_name: "".into(),
            comments: Default::default(),
        }
    }
}
impl RawSize for VorbisCommentBlock {
    fn raw_size(&self) -> usize {
        let mut comment_len = 0;
        self.comments
            .iter()
            .for_each(|(k, v)| {
                comment_len += 4 * v.len(); // 4 -> value len in bytes, times nb value for key
                comment_len += k.len() * v.len();
                for s in v {
                    comment_len += 1 + s.len()
                }
            });
        4 + self.vendor_name.len() + 4 + comment_len
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut (self.vendor_name.len() as u32).to_le_bytes().to_vec());
        bytes.append(&mut self.vendor_name.clone().into_bytes());
        bytes.append(&mut (self.comments.len() as u32).to_le_bytes().to_vec());
        for (k, v) in self.comments.iter() {
            for value in v {
                let format = format!("{}={}", k, value);
                //println!("format : {}", format);
                bytes.append(&mut (format.len() as u32).to_le_bytes().to_vec());
                bytes.append(&mut format.into_bytes());
            }

        }
        bytes
    }
}
impl VorbisCommentBlock {
    pub(crate) fn get_title(&self) -> Option<String> {
        self.comments.get("TITLE")
    }

    pub(crate) fn get_artist(&self) -> Option<String> {
        self.comments.get("ARTIST")
    }

    pub(crate) fn get_album(&self) -> Option<String> {
        self.comments.get("ALBUM")
    }

    pub(crate) fn get_album_artist(&self) -> Option<String> {
        self.comments.get("ALBUMARTIST")
    }

    pub(crate) fn get_genre(&self) -> Option<String> {
        self.comments.get("GENRE")
    }

    pub(crate) fn get_copyright(&self) -> Option<String> {
        self.comments.get("COPYRIGHT")
    }

    pub(crate) fn get_date(&self) -> Option<String> {
        self.comments.get("DATE") 
    }
    pub(crate) fn get_composer(&self) -> Option<String> {
        self.comments.get("COMPOSER")
    }
    pub(crate) fn get_track_position(&self) -> Option<String> {
        self.comments.get("TRACKNUMBER")
    }

    pub(crate) fn get_disc(&self) -> Option<String> {
        self.comments.get("DISCNUMBER")
    }

    pub(crate) fn get_comments(&self) -> Option<String> {
        self.comments.get("COMMENT")
    }

    pub(crate) fn get_disc_id(&self) -> Option<String> {
        self.comments.get("DISCID")
    }

    pub(crate) fn get_organisation(&self) -> Option<String> {
        self.comments.get("ORGANIZATION")
    }

    pub(crate) fn total_track(&self) -> Option<String> {
        self.comments.get("TRACKTOTAL")
    }
    pub(crate) fn total_disc(&self) -> Option<String> {
        self.comments.get("DISCTOTAL")
    }

    pub(crate) fn get_custom_field(&self, field: &str) -> Option<String> {
        self.comments.get(field).and_then(|s| Some(s))
    }

    pub(crate) fn get_bpm(&self) -> Option<String> {
        self.comments.get("BPM")
    }
}

impl VorbisCommentBlock {
    pub(crate) fn set_title(&mut self, content: &str) {
        self.comments.set("TITLE", content);
    }
    pub(crate) fn remove_title(&mut self) {
        self.comments.remove("TITLE");
    }
    pub(crate) fn set_album(&mut self, content: &str) {
        self.comments.set("ALBUM", content);
    }
    pub(crate) fn remove_album(&mut self) {
        self.comments.remove("ALBUM");
    }
    pub(crate) fn set_artist(&mut self, content: &str) {
        self.comments.set("ARTIST", content);
    }
    pub(crate) fn remove_artist(&mut self) {
        self.comments.remove("ARTIST");
    }
    pub(crate) fn add_artist(&mut self, content: &str) {
        self.comments.add("ARTIST", content)
    }
    pub(crate) fn set_album_artist(&mut self, content: &str) {
        self.comments.set("ALBUMARTIST", content);
    }
    pub(crate) fn remove_album_artist(&mut self) {
        self.comments.remove("ALBUMARTIST");
    }
    pub (crate) fn add_album_artist(&mut self, content: &str) {
        self.comments.add("ALBUMARTIST", content)
    }
    pub (crate) fn set_bpm(&mut self, content: u16){
        self.comments.set("BPM", content.to_string().as_str())
    }
    pub (crate) fn remove_bpm(&mut self) {
        self.comments.remove("BPM")
    }
    pub(crate) fn set_genre(&mut self, content: &str) {
        self.comments.set("GENRE", content);
    }
    pub(crate) fn add_genre(&mut self, content: &str) {
        self.comments.add("GENRE", content)
    }
    pub(crate) fn remove_genre(&mut self) {
        self.comments.remove("GENRE");
    }
    pub(crate) fn set_copyright(&mut self, content: &str) {
        self.comments.set("COPYRIGHT", content);
    }
    pub(crate) fn remove_copyright(&mut self) {
        self.comments.remove("COPYRIGHT");
    }
    pub(crate) fn set_date(&mut self, content: &str) {
        self.comments.set("DATE", content);
    }
    pub(crate) fn remove_date(&mut self) {
        self.comments.remove("DATE");
    }
    pub(crate) fn set_composer(&mut self, content: &str) {
        self.comments.set("COMPOSER", content);
    }
    pub(crate) fn remove_composer(&mut self) {
        self.comments.remove("COMPOSER");
    }
    pub (crate) fn add_composer(&mut self, content: &str){
        self.comments.add("COMPOSER", content)
    }
    pub(crate) fn set_track_position(&mut self, content: u16) {
        self.comments
            .set("TRACKNUMBER", content.to_string().as_str());
    }
    pub(crate) fn remove_track_position(&mut self) {
        self.comments.remove("TRACKNUMBER");
    }
    pub(crate) fn set_disc(&mut self, content: u16) {
        self.comments
            .set("DISCNUMBER", content.to_string().as_str());
    }
    pub(crate) fn remove_disc(&mut self) {
        self.comments.remove("DISCNUMBER");
    }
    pub(crate) fn set_comment(&mut self, content: &str) {
        self.comments.set("COMMENT", content);
    }
    pub(crate) fn remove_comments(&mut self) {
        self.comments.remove("COMMENT");
    }
    pub (crate) fn add_comment(&mut self, content: &str){
        self.comments.add("COMMENT", content)
    }
    pub(crate) fn set_organisation(&mut self, content: &str) {
        self.comments.set("ORGANIZATION", content);
    }
    pub(crate) fn remove_organisation(&mut self) {
        self.comments.remove("ORGANIZATION");
    }
    pub(crate) fn set_total_track(&mut self, content: u16) {
        self.comments
            .set("TRACKTOTAL", content.to_string().as_str());
    }
    pub(crate) fn remove_total_track(&mut self) {
        self.comments.remove("TRACKTOTAL");
    }
    pub(crate) fn set_total_disc(&mut self, content: u16) {
        self.comments
            .set("DISCTOTAL", content.to_string().as_str());
    }
    pub(crate) fn remove_total_disc(&mut self) {
        self.comments.remove("DISCTOTAL");
    }
    pub (crate) fn set_custom_field(&mut self, field : &str, content: &str){
        self.comments.set(field, content);
    }
    pub (crate) fn remove_custom_field(&mut self, field : &str) /*-> Option<String>*/ {
        self.comments.remove(field)
    }
}

pub(crate) struct PictureBlock {
    picture_type: PictureType,
    mime_type: String,
    description: String,
    pict_width: u32,
    pict_height: u32,
    color_depth: u32,
    number_of_color: u32,
    pict_data: Vec<u8>,
}
impl PictureBlock {
    pub(crate) fn new(
        picture_type: PictureType,
        mime_type: &str,
        description: Option<&str>,
        picture_width: u32,
        picture_height: u32,
        color_depth: u32,
        number_color_used: Option<u32>,
        picuture_data: &Vec<u8>,
    ) -> Self {
        let description = match description {None => "".to_owned(), Some(s) => s.into()};
        let number_of_color = match number_color_used {None => 0, Some(n) => n};
        let mime_type = format!("image/{}", mime_type);
        Self {
            picture_type,
            mime_type,
            description,
            pict_width: picture_width,
            pict_height: picture_height,
            color_depth,
            number_of_color,
            pict_data: picuture_data.clone()
        }
    }

    pub(crate) fn get_picture_data(&self) -> &Vec<u8> {
        &self.pict_data
    }
}

impl RawSize for PictureBlock {
    fn raw_size(&self) -> usize {
        4 + 4 + self.mime_type.len() + 4 + self.description.len() + 4 + 4 + 4 + 4 + 4 + self.pict_data.len()
        //self.raw_bytes().len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut (self.picture_type as u32).to_be_bytes().to_vec());
        bytes.append(&mut (self.mime_type.len() as u32).to_be_bytes().to_vec());
        bytes.append(&mut self.mime_type.clone().into_bytes());
        bytes.append(&mut (self.description.len() as u32).to_be_bytes().to_vec());
        bytes.append(&mut self.description.clone().into_bytes());
        bytes.append(&mut self.pict_width.to_be_bytes().to_vec());
        bytes.append(&mut self.pict_height.to_be_bytes().to_vec());
        bytes.append(&mut self.color_depth.to_be_bytes().to_vec());
        bytes.append(&mut self.number_of_color.to_be_bytes().to_vec());
        bytes.append(&mut (self.pict_data.len() as u32).to_be_bytes().to_vec());
        bytes.append(&mut self.pict_data.clone());
        bytes
    }
}

pub(crate) enum FlacMetadataBlockData {
    STREAMINFO(StreamInfoBlock),
    PADDING(PaddingBlock),
    APPLICATION(ApplicationBlock),
    SEEKTABLE(SeekTableBlock),
    VORBISCOMMENT(VorbisCommentBlock),
    CUESHEET(CueSheetBlock),
    PICTURE(PictureBlock),
}

impl RawSize for FlacMetadataBlockData {
    fn raw_size(&self) -> usize {
        match self {
            Self::STREAMINFO(value) => value.raw_size(),
            Self::PADDING(value) => value.raw_size(),
            Self::APPLICATION(value) => value.raw_size(),
            Self::SEEKTABLE(value) => value.raw_size(),
            Self::VORBISCOMMENT(value) => value.raw_size(),
            Self::CUESHEET(value) => value.raw_size(),
            Self::PICTURE(value) => value.raw_size(),
        }
    }

    fn raw_bytes(&self) -> Vec<u8> {
        match self {
            Self::STREAMINFO(value) => value.raw_bytes(),
            Self::PADDING(value) => value.raw_bytes(),
            Self::APPLICATION(value) => value.raw_bytes(),
            Self::SEEKTABLE(value) => value.raw_bytes(),
            Self::VORBISCOMMENT(value) => value.raw_bytes(),
            Self::CUESHEET(value) => value.raw_bytes(),
            Self::PICTURE(value) => value.raw_bytes(),
        }
    }
}

impl FlacMetadataBlockData {
    pub(crate) fn default_from(block_type: &FlacMetadataBlockType) -> Self {
        match block_type {
            FlacMetadataBlockType::STREAMINFO => todo!(),
            FlacMetadataBlockType::PADDING => Self::PADDING(PaddingBlock::default()),
            FlacMetadataBlockType::APPLICATION => todo!(),
            FlacMetadataBlockType::SEEKTABLE => todo!(),
            FlacMetadataBlockType::VORBISCOMMENT => {
                Self::VORBISCOMMENT(VorbisCommentBlock::default())
            }
            FlacMetadataBlockType::CUESHEET => todo!(),
            FlacMetadataBlockType::PICTURE => todo!(),
        }
    }
    pub(crate) fn new(
        buffer: &mut Vec<u8>,
        size: u32,
        block_type: &FlacMetadataBlockType,
    ) -> Option<Self> {
        match block_type {
            FlacMetadataBlockType::STREAMINFO => {
                let min_block_size =
                    u16::from_be_bytes(buffer.drain(0..2).collect::<Vec<u8>>().try_into().ok()?);
                //println!("minimun block size : {}", min_block_size);
                let max_block_size =
                    u16::from_be_bytes(buffer.drain(0..2).collect::<Vec<u8>>().try_into().unwrap());
                //println!("maximun block size : {}", max_block_size);
                let min_frame_size =
                    u24::from_be_bytes(buffer.drain(0..3).collect::<Vec<u8>>().try_into().unwrap());
                //println!("minimun frame size : {}", min_frame_size.value());
                let max_frame_size =
                    u24::from_be_bytes(buffer.drain(0..3).collect::<Vec<u8>>().try_into().unwrap());
                //println!("maximun frame size : {}", max_frame_size.value());
                let sample_rate_nb_chan_bit_sample_nb_sample =
                    u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().unwrap());
                let md5_signature = u128::from_be_bytes(
                    buffer.drain(0..16).collect::<Vec<u8>>().try_into().unwrap(),
                );
                //println!("signature : {}", md5_signature);
                Some(Self::STREAMINFO(StreamInfoBlock {
                    min_block_size,
                    max_block_size,
                    min_frame_size,
                    max_frame_size,
                    sample_rate_nb_chan_bit_sample_nb_sample,
                    md5_signature,
                }))
            }
            FlacMetadataBlockType::PADDING => {
                let size = size as usize;
                buffer.drain(0..size);
                //println!("padding size : {}", size);
                Some(Self::PADDING(PaddingBlock { nb_bytes: size }))
            }
            FlacMetadataBlockType::APPLICATION => {
                let app_id_name = buffer.drain(0..4).collect::<Vec<u8>>().to_utf8()?;
                //println!("application name : {}", app_id_name);
                let app_id = ApplicationID::from_str(&app_id_name.as_str()).ok()?;
                let data = buffer.drain(0..(size as usize - 4)).collect::<Vec<u8>>();
                Some(Self::APPLICATION(ApplicationBlock { app_id, data }))
            }
            FlacMetadataBlockType::SEEKTABLE => {
                let nb_seek_point = size / 18;
                let seek_point_table = SeekTableBlock::new(buffer, nb_seek_point)?;
                Some(Self::SEEKTABLE(seek_point_table))
            }
            FlacMetadataBlockType::VORBISCOMMENT => {
                let vendor_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()?;
                let vendor_str = buffer
                    .drain(0..vendor_len as usize)
                    .collect::<Vec<u8>>()
                    .to_utf8()?;
                let comment_list_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()?;
                let mut vorbis_vector = VorbisVector::new();
                for _ in 0..comment_list_len {
                    let str_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()? as usize;
                    let comment = buffer.drain(0..str_len).collect::<Vec<u8>>().to_utf8()?;
                    //println!("comment : {}", comment);
                    let mut split = comment.splitn(2, "=");
                    let first = split.next()?;
                    let second = split.next()?;
                    //println!("  {} : {} ", first, second);

                    vorbis_vector.add(first, second);
                }
                //println!("In Vorbis data size passed : {}", size);
                Some(Self::VORBISCOMMENT(VorbisCommentBlock {
                    vendor_name: vendor_str,
                    comments: vorbis_vector,
                }))
            }
            FlacMetadataBlockType::PICTURE => {
                let picture_type_n = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let picture_type = PictureType::from_raw_u32(picture_type_n)?;
                let mime_type_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let mime_type = buffer
                    .drain(0..(mime_type_len as usize))
                    .collect::<Vec<u8>>()
                    .to_utf8()?;
                let description_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let description = buffer
                    .drain(0..(description_len as usize))
                    .collect::<Vec<u8>>()
                    .to_utf8()?;
                let pict_width = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let pict_height = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let color_depth = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let number_of_color = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let _ = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let size = size - (4 + 4 + mime_type_len + 4 + description_len + 20);
                let pict_data = buffer.drain(0..(size as usize)).collect::<Vec<u8>>();
                // println!(
                //     "width : {}\nheight : {}\ncolor_depth : {}\nnumber_of_color : {}",
                //     pict_width, pict_height, color_depth, number_of_color
                // );
                Some(Self::PICTURE(PictureBlock {
                    picture_type,
                    mime_type,
                    description,
                    pict_width,
                    pict_height,
                    color_depth,
                    number_of_color,
                    pict_data,
                }))
            }
            FlacMetadataBlockType::CUESHEET => {
                let media_catalog = buffer.drain(0..128).collect::<Vec<u8>>().to_utf8()?;
                let number_lead_sample =
                    u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
                let is_compact_disc = (buffer.remove(0) & COMPACT_DISC_FLAG) == COMPACT_DISC_FLAG;
                buffer.drain(0..258);
                let number_of_tracks = buffer.remove(0);
                //println!("number of cuesheet : {}", number_of_tracks);
                let mut cuesheets_tracks = vec![];
                for _ in 0..number_of_tracks {
                    let cuesheet = CueSheetTrack::new(buffer)?;
                    cuesheets_tracks.push(cuesheet);
                }
                Some(Self::CUESHEET(CueSheetBlock {
                    media_catalog_number: media_catalog,
                    number_lead_sample,
                    is_compact_disc,
                    cuesheets_tracks,
                }))
            }
        }
    }
}

impl FlacMetadataBlockData {
    pub(crate) fn as_vorbis_comments_block(&self) -> Option<&VorbisCommentBlock> {
        match self {
            FlacMetadataBlockData::VORBISCOMMENT(vc) => Some(vc),
            _ => None,
        }
    }

    pub(crate) fn as_vorbis_comments_block_mut(&mut self) -> Option<&mut VorbisCommentBlock> {
        match self {
            FlacMetadataBlockData::VORBISCOMMENT(vc) => Some(vc),
            _ => None,
        }
    }

    pub(crate) fn as_picture_block(&self) -> Option<&PictureBlock> {
        match self {
            Self::PICTURE(pc) => Some(pc),
            _ => None,
        }
    }
}
