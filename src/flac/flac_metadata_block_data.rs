use crate::{util::number::u24, util::traits::{ToU32, StringConvert, ToU16}, id3::code::picture_code::picture_type::PictureType};
use std::{fmt::Display, str::FromStr, collections::HashMap, convert::TryInto};

use super::flac_metadata_block::FlacMetadataBlockType;

const COMPACT_DISC_FLAG : u8 = 0b10_000_000;

const TRACK_TYPE_MASK : u8 = 0b10_000_000;
const PRE_EMPHASIS_MASK: u8 = 0b01_000_000;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub (crate) enum ApplicationID  {
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
    aiff,
    imag,
    peem,
    qfst,
    riff,
    tune,
    xbat,
    xmcd
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
            "aiff" => Ok(Self::aiff),
            "imag" => Ok(Self::imag),
            "peem" => Ok(Self::peem),
            "qfst" => Ok(Self::qfst),
            "riff" => Ok(Self::riff),
            "tune" => Ok(Self::tune),
            "xbat" => Ok(Self::xbat),
            "xmcd" => Ok(Self::xmcd),
            _  => Err(())
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
            Self::aiff => "aiff",
            Self::imag => "imag",
            Self::peem => "peem",
            Self::qfst => "qfst",
            Self::riff => "riff",
            Self::tune => "tune",
            Self::xbat => "xbat",
            Self::xmcd => "xmcd",
        };
        write!(f, "{}", s)
    }
}

pub (crate) struct CueSheetTrackIndex {
    offset : u64,
    index_point_number : u8,
}

impl CueSheetTrackIndex {
    pub(crate) fn new(buffer : &mut Vec<u8>) -> Option<Self> {
        let offset = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let index_point_number = buffer.remove(0);
        Some(
            Self {
                offset,
                index_point_number
            }
        )
    }
}


pub (crate) struct CueSheetTrack {
    track_offset : u64,
    track_number : u8, // not 0 
    track_isrc : Vec<u8>,
    is_audio : bool,  
    is_pre_emphasis : bool, // 13 null bytes
    number_track_index_point : u8,
    tracks_index : Vec<CueSheetTrackIndex>
}

impl CueSheetTrack {
    pub(crate) fn new(buffer: &mut Vec<u8>) -> Option<Self>  {
       let track_offset = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
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
       Some(Self{
           track_offset,
           track_number,
           track_isrc,
           is_audio,
           is_pre_emphasis,
           number_track_index_point,
           tracks_index
       })

    }
}

 

pub (crate) struct StreamInfoBlock {
    min_block_size : u16,
    max_block_size : u16,
    min_frame_size : u24,
    max_frame_size : u24,
    sample_rate_nb_chan_bit_sample_nb_sample : u64,
    md5_signature : u128
}


pub (crate) struct PaddingBlock {
    nb_bytes : usize
}


pub(crate) struct ApplicationBlock {
    app_id : ApplicationID,
    data : Vec<u8>
}
pub struct SeekPoint {
    sample_number_of_first_sample : u64,
    offset : u64,
    number_of_sample : u16
}

impl SeekPoint {
    fn new(buffer : &mut Vec<u8>) -> Option<Self> {
        let sample_number_of_first_sample = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let offset = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
        let number_of_sample = buffer.drain(0..2).collect::<Vec<u8>>().u16_from_be()?;
        Some( Self {
            sample_number_of_first_sample,
            offset,
            number_of_sample
        })
    }
}

pub (crate) struct SeekTableBlock {
    seek_points : Vec<SeekPoint>
}

impl SeekTableBlock {
    fn new(buffer : &mut Vec<u8>, nb_to_create : u32) -> Option<Self> {
        let mut seek_points = vec![];
        for _ in 0..nb_to_create {
            let seek_point = SeekPoint::new(buffer)?;
            seek_points.push(seek_point);
        }
        Some (
            Self {
                seek_points
            }
        )
    }
}

pub (crate) struct VorbisCommentBlock {
    vendor_str_len : u32,
    vendor_name : String,
    comments_number : u32,
    comments : HashMap<String, String>
}
impl VorbisCommentBlock {
    pub(crate) fn _comments(&self) -> &HashMap<String, String> {
        &self.comments
    }

    pub (crate) fn get_title(&self) -> Option<String> {
        if let Some(title) = self.comments.get("TITLE"){
            Some(title.clone())
        }else if let Some(title) = self.comments.get("Title") {
            Some(title.clone())
        }else if let Some(title) = self.comments.get("title") {
            Some(title.clone())
        }else {
            None
        }
    }

    pub(crate) fn get_artist(&self) -> Option<String> {
        if let Some(artist) = self.comments.get("ARTIST"){
            Some(artist.clone())
        }else if let Some(artist) = self.comments.get("Artist") {
            Some(artist.clone())
        }else if let Some(artist) = self.comments.get("artist") {
            Some(artist.clone())
        }else {
            None
        }
    }

    pub (crate) fn get_album(&self) -> Option<String> {
        if let Some(album) = self.comments.get("ALBUM"){
            Some(album.clone())
        }else if let Some(album) = self.comments.get("Album") {
            Some(album.clone())
        }else if let Some(album) = self.comments.get("album") {
            Some(album.clone())
        }else {
            None
        }
    }
    
    pub (crate) fn get_album_artist(&self) -> Option<String> {
        if let Some(album_artist) = self.comments.get("ALBUMARTIST"){
            Some(album_artist.clone())
        }else if let Some(album_artist) = self.comments.get("AlbumArtist") {
            Some(album_artist.clone())
        }else if let Some(album_artist) = self.comments.get("Albumartist") {
            Some(album_artist.clone())
        }else if let Some(album_artist) = self.comments.get("albumartist") {
            Some(album_artist.clone())
        }else {
            None
        }
    }


}

pub (crate) struct CueSheetBlock {
    media_catalog_number : String, // 128 bytes length
    number_lead_sample : u64,
    is_compact_disc : bool, // 1 + 258 bytes
    number_of_tracks : u8, // 1..100
    cuesheets_tracks : Vec<CueSheetTrack>
}

pub (crate) struct PictureBlock {
    picture_type : PictureType,
    mime_type_len: u32,
    mime_type : String,
    description_len : u32,
    description : String,
    pict_width : u32,
    pict_height : u32,
    color_depth : u32,
    number_of_color : u32,
    data_len : u32,
    pict_data : Vec<u8>
}


pub (crate) enum FlacMetadataBlockData {
    STREAMINFO(StreamInfoBlock),
    PADDING(PaddingBlock),
    APPLICATION(ApplicationBlock),
    SEEKTABLE(SeekTableBlock),
    VORBISCOMMENT(VorbisCommentBlock),
    CUESHEET(CueSheetBlock),
    PICTURE(PictureBlock),
}

impl FlacMetadataBlockData {
    pub (crate) fn new(buffer : &mut Vec<u8>, size : u32, block_type : &FlacMetadataBlockType ) -> Option<Self> {
        match block_type{
            FlacMetadataBlockType::STREAMINFO => {
                let min_block_size = u16::from_be_bytes(buffer.drain(0..2).collect::<Vec<u8>>().try_into().ok()?);
                println!("minimun block size : {}", min_block_size);
                let max_block_size = u16::from_be_bytes(buffer.drain(0..2).collect::<Vec<u8>>().try_into().unwrap());
                println!("maximun block size : {}", max_block_size);
                let min_frame_size = u24::from_be_bytes(buffer.drain(0..3).collect::<Vec<u8>>().try_into().unwrap());
                println!("minimun frame size : {}", min_frame_size.value());
                let max_frame_size = u24::from_be_bytes(buffer.drain(0..3).collect::<Vec<u8>>().try_into().unwrap());
                println!("maximun frame size : {}", max_frame_size.value());
                let sample_rate_nb_chan_bit_sample_nb_sample = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().unwrap());
                let md5_signature = u128::from_be_bytes(buffer.drain(0..16).collect::<Vec<u8>>().try_into().unwrap());
                println!("signature : {}", md5_signature);
                Some( Self::STREAMINFO(
                    StreamInfoBlock {
                        min_block_size,
                        max_block_size,
                        min_frame_size,
                        max_frame_size,
                        sample_rate_nb_chan_bit_sample_nb_sample,
                        md5_signature
                }) 
            )
            },
            FlacMetadataBlockType::PADDING => {
                let size = size as usize;
                buffer.drain(0..size);
                println!("padding size : {}", size);
                Some(Self::PADDING(PaddingBlock {
                    nb_bytes : size
                }))
            },
            FlacMetadataBlockType::APPLICATION => {
                let app_id_name = buffer.drain(0..4).collect::<Vec<u8>>().to_utf8()?;
                println!("application name : {}", app_id_name);
                let app_id = ApplicationID::from_str(&app_id_name.as_str()).ok()?;
                let data = buffer.drain(0..(size as usize - 4)).collect::<Vec<u8>>();
                Some(
                    Self::APPLICATION(ApplicationBlock {
                        app_id,
                        data
                    })
                )
            },
            FlacMetadataBlockType::SEEKTABLE => {
                let nb_seek_point = size / 18;
                let seek_point_table = SeekTableBlock::new(buffer, nb_seek_point)?;
                Some(Self::SEEKTABLE(seek_point_table))
            },
            FlacMetadataBlockType::VORBISCOMMENT => {
                let vendor_len  = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()?;
                let vendor_str = buffer.drain(0..vendor_len as usize).collect::<Vec<u8>>().to_utf8()?;
                println!("vendor name : {}", vendor_str);
                let comment_list_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()?;
                println!("number od comment : {}", comment_list_len);
                let mut hash : HashMap<String, String> = HashMap::new();
                for _ in 0..comment_list_len {
                    
                    let str_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_le()? as usize;
                    let comment = buffer.drain(0..str_len).collect::<Vec<u8>>().to_utf8()?;
                    //println!("comment : {}", comment);
                    let mut split = comment.splitn(2, "=");
                    let first = split.next()?;
                    let second = split.next()?;
                    println!("  {} : {} ", first, second);
                    
                    hash.insert(first.into(), second.into());
                }
                //println!("{:?}", hash);
                Some(
                    Self::VORBISCOMMENT(
                        VorbisCommentBlock {
                            vendor_str_len: vendor_len,
                            vendor_name: vendor_str,
                            comments_number: comment_list_len,
                            comments: hash
                        }
                    )
                )
            },
            FlacMetadataBlockType::PICTURE => {
                let picture_type_n = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let picture_type = PictureType::from_raw_u32(picture_type_n)?;
                let mime_type_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let mime_type = buffer.drain(0..(mime_type_len as usize)).collect::<Vec<u8>>().to_utf8()?;
                let description_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let description = buffer.drain(0..(description_len as usize)).collect::<Vec<u8>>().to_utf8()?;
                let pict_width = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let pict_height = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let color_depth = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let number_of_color = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let data_len = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be()?;
                let size = size - (4 + 4 + mime_type_len + 4 + description_len + 20);
                let pict_data = buffer.drain(0..(size as usize)).collect::<Vec<u8>>();
                Some(
                    Self::PICTURE(PictureBlock {
                        picture_type,
                        mime_type_len,
                        mime_type,
                        description_len,
                        description,
                        pict_width,
                        pict_height,
                        color_depth,
                        number_of_color,
                        data_len,
                        pict_data,
                    })
                )
            },
            FlacMetadataBlockType::CUESHEET => {
                let media_catalog = buffer.drain(0..128).collect::<Vec<u8>>().to_utf8()?;
                let number_lead_sample = u64::from_be_bytes(buffer.drain(0..8).collect::<Vec<u8>>().try_into().ok()?);
                let is_compact_disc = (buffer.remove(0) & COMPACT_DISC_FLAG) == COMPACT_DISC_FLAG;
                buffer.drain(0..258);
                let number_of_tracks = buffer.remove(0);
                println!("number of cuesheet : {}", number_of_tracks);
                let mut cuesheets_tracks = vec![];
                for _ in 0..number_of_tracks {
                    let cuesheet = CueSheetTrack::new(buffer)?;
                    cuesheets_tracks.push(cuesheet);
                }
                Some(
                    Self::CUESHEET(CueSheetBlock {
                        media_catalog_number: media_catalog,
                        number_lead_sample,
                        is_compact_disc,
                        number_of_tracks,
                        cuesheets_tracks,
                    })
                )
            },
            
        }
    }
}

impl FlacMetadataBlockData {
    pub(crate) fn as_vorbis_comments_frame(&self) -> Option<&VorbisCommentBlock> {
        match self {
            FlacMetadataBlockData::VORBISCOMMENT(vc) => Some(vc),
            _ => None
        }
    }
}