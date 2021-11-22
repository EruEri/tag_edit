use crate::{tag::traits::RawSize, util::{function::{ToU16, ToU32, first_string, split_to_string_utf16, split_to_string_utf8, to_u16_le, vec_to_string}, reading_mode::TextEncoding}};

use super::{code::{event_timing_code::time_stamp_format::TimeStampFormat, picture_code::picture_type:: PictureType, text_code::content_type::TextContent}, id3_frameid::ID3FRAMEID};
use super::id3_frameid::ID3FRAMEID::*;
pub(crate) struct UniqueFileIdentifierFrame {
    owner_id : String,
    id : Vec<u8>
}
impl RawSize for UniqueFileIdentifierFrame {
    fn raw_size(&self) -> usize {
        self.owner_id.len() + self.id.len()
    }
}

pub(crate) struct TextFrame {
    text_encoding : TextEncoding,
    text : String
}
impl RawSize for TextFrame {
    fn raw_size(&self) -> usize {
        1 + if self.text_encoding.is_one_byte() {
            self.text.len() 
        } else {
            self.text.len() * 2
        }
    }
}
impl TextFrame {
    pub(crate) fn get_text(&self) -> String {
        self.text.clone()
    }
}

pub(crate) struct UserInfoFrame{
    text_encoding : TextEncoding,
    description : String,
    text : String
}
impl RawSize for UserInfoFrame {
    fn raw_size(&self) -> usize {
        let text_len = if self.text_encoding.is_one_byte() {
            self.description.len() + self.text.len()
        }else {
            (self.description.len() * 2) + (self.text.len() * 2)
        };
        1 + text_len
    }
}

pub(crate) struct UrlFrame {
    url : String
}
impl RawSize for UrlFrame {
    fn raw_size(&self) -> usize {
        self.url.len()
    }
}

pub(crate) struct InvolvedPeopleFrame {
    text_encoding : TextEncoding,
    people_list : String
}
impl RawSize for InvolvedPeopleFrame {
    fn raw_size(&self) -> usize {
        1 + if self.text_encoding.is_one_byte() {
            self.people_list.len() 
        } else {
            self.people_list.len() * 2
        }
    }
}
pub (crate) struct MusicCdIdframe {
    cd_toc : Vec<u8>
}
impl RawSize for MusicCdIdframe {
    fn raw_size(&self) -> usize {
        self.cd_toc.len()
    }
}

pub(crate) struct EventTimingFrame {
    time_stamp_format : TimeStampFormat,
    raw : Vec<u8>
}
impl RawSize for EventTimingFrame {
    fn raw_size(&self) -> usize {
        1 + self.raw.len()
    }
}

pub (crate) struct LocationLookupTableFrame {
    raw : Vec<u8>
}

impl RawSize for LocationLookupTableFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub(crate) struct  SyncTempoCodeFrame {
    time_stamp_format : TimeStampFormat,
    tempo_data : Vec<u8>
}
impl RawSize for SyncTempoCodeFrame {
    fn raw_size(&self) -> usize {
        1 + self.tempo_data.len()
    }
}
pub(crate) struct UnsyncLyricsFrame{
    text_encoding : TextEncoding,
    language : String,
    content_description : String,
    text : String
}
impl RawSize for UnsyncLyricsFrame {
    fn raw_size(&self) -> usize {
        let text_len = if self.text_encoding.is_one_byte() {
            self.content_description.len() + self.text.len()
        }else {
            (self.content_description.len() * 2) + (self.text.len() * 2)
        };
        1 + 3 + text_len
    }
}
impl UnsyncLyricsFrame {
    pub (crate) fn get_text_encoding(&self) -> &TextEncoding {
        &self.text_encoding
    }

    pub (crate) fn get_content_description(&self) -> &String {
        &self.content_description
    }

    pub (crate) fn get_lyrics(&self) -> &String {
        &self.text
    }
}
pub(crate) struct SyncLyricsFrame {
    raw : Vec<u8>
}

impl RawSize for SyncLyricsFrame {
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub (crate) struct CommentFrame {
    text_encoding : TextEncoding,
    language : String,
    content_description : String,
    text : String
}
impl RawSize for CommentFrame{
    fn raw_size(&self) -> usize {
        let text_len = if self.text_encoding.is_one_byte() {
            self.content_description.len() + self.text.len()
        }else {
            (self.content_description.len() * 2) + (self.text.len() * 2)
        };
        1 + 3 + text_len
    }
}

pub(crate) struct RelativeVolumeAdjustementFrame {
    raw : Vec<u8>
}
impl RawSize for RelativeVolumeAdjustementFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct EqualiisationFrame {
    raw : Vec<u8>
}
impl RawSize for EqualiisationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct ReverbFrame {
    raw : Vec<u8>
}
impl RawSize for ReverbFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub(crate) struct  AttachedPictureFrame {
    text_encode : TextEncoding,
    mime_type : String,
    picture_type : PictureType,
    description : String,
    picture_data : Vec<u8>
}
impl RawSize for AttachedPictureFrame{
    fn raw_size(&self) -> usize {
        let description_size = if self.text_encode.is_one_byte() {
            self.description.len()
        }else {
            self.description.len() * 2
        };
        1 + self.mime_type.len() + 1 + description_size + self.picture_data.len()
    }
}
impl AttachedPictureFrame {
    pub(crate) fn get_picture_data(&self) -> &Vec<u8>{
        &self.picture_data
    }
}

pub(crate) struct GeneralEncapsulatedObjectFrame {
    text_encoding : TextEncoding,
    mime_type : String,
    filename : String,
    content : Vec<u8>,
    encapsulated_object : Vec<u8>
}
impl RawSize for GeneralEncapsulatedObjectFrame{
    fn raw_size(&self) -> usize {
        let filename_size = if self.text_encoding.is_one_byte() {
            self.filename.len()
        }else {
            self.filename.len() * 2
        };
        1 + self.mime_type.len() + filename_size + self.content.len() + self.encapsulated_object.len()
    }
}

pub(crate) struct PlayCounterFrame {
    raw : Vec<u8>
}
impl RawSize for PlayCounterFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct PopularimeterFrame {
    raw : Vec<u8>
}
impl RawSize for PopularimeterFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct RecommendedBufferSizeFrame {
    buffer_size : u32,
    embedded_info_flag : bool,
    offet_next_tag : u32
}
impl RawSize for RecommendedBufferSizeFrame{
    fn raw_size(&self) -> usize {
        4 + 1 + 4 /* 9 lol */
    }
}

pub(crate) struct AudioEncryptionFrame {
    owner_id : String,
    preview_start : u16,
    preview_lenght : u16,
    encryption_info : Vec<u8>
}
impl RawSize for AudioEncryptionFrame {
    fn raw_size(&self) -> usize {
        self.owner_id.len() + 2 + 2 + self.encryption_info.len()
    }
}
pub (crate) struct LinkedInfoFrame {
    raw : Vec<u8>
}
impl RawSize for LinkedInfoFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub(crate) struct PositionSyncFrame {
    raw : Vec<u8>
}

impl RawSize for PositionSyncFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct TermsUseFrame {
    text_encoding : TextEncoding,
    language : String,
    text : String
}
impl RawSize for TermsUseFrame {
    fn raw_size(&self) -> usize {
        1 + 3 + if self.text_encoding.is_one_byte() {
            self.text.len() 
        } else {
            self.text.len() * 2
        }
    }
}

pub(crate) struct OwnershipFrame {
    raw : Vec<u8>
}
impl RawSize for OwnershipFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub (crate) struct CommercialFrame {
    raw : Vec<u8>
}
impl RawSize for CommercialFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub(crate) struct EncryptionMethodRegistationFrame {
    raw : Vec<u8>
}
impl RawSize for EncryptionMethodRegistationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}
pub(crate) struct GroupIdentificationRegistationFrame {
    raw : Vec<u8>
}
impl RawSize for GroupIdentificationRegistationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
}

pub(crate) struct PrivateFrame {
    owner_id : String,
    private_data : Vec<u8>
}

impl RawSize for PrivateFrame {
    fn raw_size(&self) -> usize {
        self.owner_id.len() + self.private_data.len()
    }
}

pub(crate) enum FrameValue {
    UFIF(UniqueFileIdentifierFrame),
    TF(TextFrame),
    UIF(UserInfoFrame),
    UF(UrlFrame),
    IPF(InvolvedPeopleFrame),
    MCIF(MusicCdIdframe),
    ETF(EventTimingFrame),
    LLTF(LocationLookupTableFrame),
    SYCF(SyncTempoCodeFrame),
    ULF(UnsyncLyricsFrame),
    SLF(SyncLyricsFrame),
    CF(CommentFrame),
    RVAF(RelativeVolumeAdjustementFrame),
    EF(EqualiisationFrame),
    RF(ReverbFrame),
    APF(AttachedPictureFrame),
    GEOF(GeneralEncapsulatedObjectFrame),
    PCF(PlayCounterFrame),
    PF(PopularimeterFrame),
    RBSF(RecommendedBufferSizeFrame),
    AEF(AudioEncryptionFrame),
    LIF(LinkedInfoFrame),
    PSF(PositionSyncFrame),
    TUF(TermsUseFrame),
    OF(OwnershipFrame),
    CommercialF(CommercialFrame),
    EMRF(EncryptionMethodRegistationFrame),
    GIRF(GroupIdentificationRegistationFrame),
    PrivF(PrivateFrame),
    Undefined(Vec<u8>),
    ICFF(i16),
    NoValue
}

impl FrameValue {
    pub (crate) fn new (buffer : &mut Vec<u8>, frame_id : ID3FRAMEID, size : u32) -> Option<Self> {
        match frame_id {
            TCMP => {
                let string = String::from_utf8(buffer.drain(0..(size as usize)).collect()).ok()?;
                println!("TCMP str : {}", string);
                let is_compilation = string.parse().ok()?;
                Some(
                    Self::ICFF(is_compilation)
                )
            }
            id if id.to_string().starts_with("T") => {
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                println!("encode : {:?}", encode);
                if id == ID3FRAMEID::TXXX {
                    let buffer_i : Vec<u8> = buffer.drain(0..((size-1) as usize)).collect();
                    let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                    let description = strings.first()?.clone();
                    let text = strings.get(1)?.clone();
                    println!("Description :=> {}\nText :=> {}", description, text);
                    Some( Self::UIF( UserInfoFrame { text_encoding: encode, description, text}))
                    //Some(Self::Undefined(buffer))
                }else {
                    let string_buff = buffer.drain(0..((size-1) as usize)).collect::<Vec<u8>>();
                    let text = vec_to_string(string_buff, &encode)?;
                    println!("Text Value :=> {}", &text);
                    let text_frame = TextFrame { text_encoding : encode, text};
                    Some( Self::TF( text_frame) )
                }
            }
            WCOM | WCOP | WOAF | WOAR | WOAS | WORS | WPAY | WPUB => {
                let url =  String::from_utf8(buffer.drain(0..((size-1) as usize)).collect()).ok()?;
                Some( Self::UF( UrlFrame { url } ))  
            },
            COMM => {
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                println!("encode : {:?}", encode);
                let language = String::from_utf8(buffer.drain(0..3).collect::<Vec<u8>>()).ok()?;
                let buffer_i : Vec<u8> = buffer.drain(0..((size-4) as usize)).collect();
                let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                let (content_description, text ) = if strings.len() == 1 {
                    (strings.first()?.clone(), "".into())
                }else {(strings.first()?.clone(), strings.get(1)?.clone() ) };
                println!("Description :=> {}\nText :=> {}", content_description, text);
                Some(Self::CF(CommentFrame { text_encoding: encode, language, content_description, text }))

            },
            APIC => {
                let start_len = buffer.len();
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                println!("encode : {:?}", encode);
                let mime_type = first_string(buffer, &encode, true)?;
                let picture_type = PictureType::from_raw_value(buffer.remove(0))?;
                println!("Mime Type : {}", mime_type);
                let description = first_string(buffer, &encode, true)?;
                println!("Description : {}", description);
                let drop_len = size as usize - (start_len - buffer.len());
                let picture_data = buffer.drain(0..drop_len).collect::<Vec<u8>>();                
                Some(Self::APF(AttachedPictureFrame {
                    text_encode: encode,
                    mime_type,
                    picture_type,
                    description,
                    picture_data
                }))
            },
            IPLS => {
                let encode = TextEncoding::from_raw_value(buffer.remove(0)).unwrap_or(TextEncoding::Iso8859_1);
                let people_list = first_string(buffer, &encode, true)?;
                Some( Self::IPF(
                        InvolvedPeopleFrame {
                            text_encoding: encode,
                            people_list }
                    )
                )
            }
            UFID => {
                let start_len = buffer.len();
                let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let drain_size  = size as usize - (start_len - buffer.len()); 
                let id = buffer.drain(0..drain_size).collect();
                Some(Self::UFIF(UniqueFileIdentifierFrame{
                    owner_id,
                    id
                }))
            }
            MCDI => {
                let data = buffer.drain(0..(size as usize)).collect();
                Some(Self::MCIF(MusicCdIdframe{
                    cd_toc: data
                }))
            }
            ETCO => {
                let time_stamp_format = TimeStampFormat::from_raw_value(buffer.remove(0))?;
                let raw = buffer.drain(0..((size-1) as usize)).collect();
                Some(Self::ETF(EventTimingFrame {
                    time_stamp_format,
                    raw
                } ))
            }
            MLLT => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::LLTF(LocationLookupTableFrame{
                    raw
                }))
            }
            SYTC => {
                let time_stamp_format = TimeStampFormat::from_raw_value(buffer.remove(0))?;
                let tempo_data = buffer.drain(0..((size-1) as usize)).collect();
                Some(Self::SYCF(SyncTempoCodeFrame {
                    time_stamp_format,
                    tempo_data
                } ))
            }
            SYLT => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some(Self::SLF(SyncLyricsFrame{
                    raw
                }))

            }
            USLT => {
                let encode = TextEncoding::from_raw_value(buffer.remove(0)).unwrap_or(TextEncoding::Iso8859_1);
                println!("encode : {:?}", encode);
                let language = String::from_utf8(buffer.drain(0..3).collect::<Vec<u8>>()).ok()?;
                let buffer_i : Vec<u8> = buffer.drain(0..((size-4) as usize)).collect();
                let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                let (content_description, text ) = if strings.len() == 1 {
                    (strings.first()?.clone(), "".into())
                }else {(strings.first()?.clone(), strings.get(1)?.clone() ) };
                println!("Description :=> {}\nText :=> {}", content_description, text);
                Some(Self::ULF(UnsyncLyricsFrame 
                    { text_encoding: encode, language, content_description, text }
                ))
            }
            RVAD => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::RVAF(RelativeVolumeAdjustementFrame{
                    raw
                }))
            }
            EQUA => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::EF(EqualiisationFrame{
                    raw
                }))
            }
            RVRB => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::RF(ReverbFrame{
                    raw
                }))
            }
            GEOB => {
                let start_size = buffer.len();
                let encode = TextEncoding::from_raw_value(buffer.remove(0)).unwrap_or(TextEncoding::Iso8859_1);
                let mime_type = first_string(buffer, &encode, true)?;
                let filename = first_string(buffer, &encode, true)?;
                let content = if encode.is_one_byte() {
                    buffer.drain(0..1).collect()
                }else {
                    buffer.drain(0..2).collect()
                };
                let end_size = start_size - buffer.len();
                let encapsulated_object = buffer.drain(0..( (size - end_size as u32) as usize)).collect();
                Some(Self::GEOF(GeneralEncapsulatedObjectFrame {
                    text_encoding: encode,
                    mime_type,
                    filename,
                    content,
                    encapsulated_object
                }))
            }
            PCNT => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::PCF(PlayCounterFrame{
                    raw
                }))
            }
            POPM => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::PF(PopularimeterFrame{
                    raw
                }))
            }
            RBUF => {
                let buffer_size = buffer.drain(0..4).collect::<Vec<u8>>().to_u32_be().unwrap();
                let embedded_info = buffer.remove(0) == 1;
                let offset = buffer.drain(0..4).collect::<Vec<u8>>().to_u32_be().unwrap();
                Some(Self::RBSF(RecommendedBufferSizeFrame {
                    buffer_size,
                    embedded_info_flag: embedded_info,
                    offet_next_tag: offset
                }))
            }
            AENC => {
                let start_size = buffer.len();
                let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let preview_start  = buffer.drain(0..2).collect::<Vec<u8>>().to_u16_be()?;
                let preview_lenght  = buffer.drain(0..2).collect::<Vec<u8>>().to_u16_be()?;
                let end_size = start_size - buffer.len();
                let encryption_info = buffer.drain(0..(size as usize - end_size )).collect();
                Some(Self::AEF(AudioEncryptionFrame {
                    owner_id,
                    preview_start,
                    preview_lenght,
                    encryption_info
                }))
            }
            LINK => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::LIF(LinkedInfoFrame{
                    raw
                }))
            }
            POSS => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::PSF(PositionSyncFrame{
                    raw
                }))
            }
            USER => {
                let encode = TextEncoding::from_raw_value(buffer.remove(0)).unwrap_or(TextEncoding::Iso8859_1);
                let language = String::from_utf8(buffer.drain(0..3).collect()).ok()?;
                let text = vec_to_string(buffer.drain(0..(size as usize - 4)).collect()
                , &encode)?;
                Some(Self::TUF(TermsUseFrame {
                    text_encoding: encode,
                    language,
                    text
                }))

            } 
            OWNE => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::OF(OwnershipFrame{
                    raw
                }))
            }
            COMR => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::CommercialF(CommercialFrame{
                    raw
                }))
            }
            ENCR => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::EMRF(EncryptionMethodRegistationFrame{
                    raw
                }))
            }
            GRID => {
                let raw = buffer.drain(0..(size as usize)).collect();
                Some( Self::GIRF(GroupIdentificationRegistationFrame{
                    raw
                }))
            }
            PRIV => {
                let start_len = buffer.len();
                let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let end_len = start_len - buffer.len();
                let private_data = buffer.drain(0..(size as usize - end_len)).collect();
                Some(Self::PrivF(PrivateFrame {
                    owner_id,
                    private_data
                }))
            }
            _ =>  Some( Self::Undefined(buffer.drain(0..(size as usize)).collect()) )
        }
    }
}

impl RawSize for FrameValue {
    fn raw_size(&self) -> usize {
        match self{
            FrameValue::UFIF(ufif) => ufif.raw_size(),
            FrameValue::TF(fv) => fv.raw_size(),
            FrameValue::UIF(fv) => fv.raw_size(),
            FrameValue::UF(fv) => fv.raw_size(),
            FrameValue::IPF(fv) => fv.raw_size(),
            FrameValue::MCIF(fv) => fv.raw_size(),
            FrameValue::ETF(fv) => fv.raw_size(),
            FrameValue::LLTF(fv) => fv.raw_size(),
            FrameValue::SYCF(fv) => fv.raw_size(),
            FrameValue::ULF(fv) => fv.raw_size(),
            FrameValue::SLF(fv) => fv.raw_size(),
            FrameValue::CF(fv) => fv.raw_size(),
            FrameValue::RVAF(fv) => fv.raw_size(),
            FrameValue::EF(fv) => fv.raw_size(),
            FrameValue::RF(fv) => fv.raw_size(),
            FrameValue::APF(fv) => fv.raw_size(),
            FrameValue::GEOF(fv) => fv.raw_size(),
            FrameValue::PCF(fv) => fv.raw_size(),
            FrameValue::PF(fv) => fv.raw_size(),
            FrameValue::RBSF(fv) => fv.raw_size(),
            FrameValue::AEF(fv) => fv.raw_size(),
            FrameValue::LIF(fv) => fv.raw_size(),
            FrameValue::PSF(fv) => fv.raw_size(),
            FrameValue::TUF(fv) => fv.raw_size(),
            FrameValue::OF(fv) => fv.raw_size(),
            FrameValue::CommercialF(fv) => fv.raw_size(),
            FrameValue::EMRF(fv) => fv.raw_size(),
            FrameValue::GIRF(fv) => fv.raw_size(),
            FrameValue::PrivF(fv) => fv.raw_size(),
            FrameValue::Undefined(raw) => raw.len(),
            FrameValue::ICFF(_) => 3,
            Self::NoValue => 0,
        }
    }
}


impl FrameValue {
    pub fn as_attached_picture_frame(&self) -> Option<&AttachedPictureFrame>{
        match self {
            Self::APF(picture_frame) => Some(picture_frame),
            _ => None
        }
    }
    pub fn as_text_frame(&self) -> Option<&TextFrame>{
        match self {
            Self::TF(tf) => Some(tf),
            _ => None
        }
    }

    pub fn as_unsynchroned_lyrics_frame(&self) -> Option<&UnsyncLyricsFrame>{
        match self {
            Self::ULF(f) => Some(f),
            _ => None
        }
    }
}