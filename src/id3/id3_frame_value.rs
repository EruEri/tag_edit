use crate::util::{traits::{RawSize, ToBytes, StringConvert, SplitString, ToU32, ToU16}, reading_mode::{TextEncoding, NULL_TERMINATE}, file_format::PictureFormat};

use super::{code::{event_timing_code::time_stamp_format::TimeStampFormat, picture_code::picture_type:: PictureType}, id3_frameid::ID3FRAMEID};
use super::id3_frameid::ID3FRAMEID::*;



pub(crate) struct UniqueFileIdentifierFrame {
    owner_id : String,
    id : Vec<u8>
}
impl RawSize for UniqueFileIdentifierFrame {
    fn raw_size(&self) -> usize {
        self.raw_bytes().len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.owner_id.clone().into_bytes());
        bytes.push(NULL_TERMINATE);
        bytes.append(&mut self.id.clone());
        bytes
    }
}

pub(crate) struct TextFrame {
    text_encoding : TextEncoding,
    text : String
}
impl RawSize for TextFrame {
    fn raw_size(&self) -> usize {
        self.raw_bytes().len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.text.to_bytes(&self.text_encoding, false));
        bytes
    }
}
impl TextFrame {

    pub (crate) fn new(tag_version: u8, text: String) -> Self {
        let text_encoding = if text.is_ascii() { 
            TextEncoding::Iso8859_1 
        } else if tag_version == 3 {
            TextEncoding::UnicodeUtf16
        } else {
            TextEncoding::UnicodeUtf8
         };
         Self {
             text_encoding,
             text
         }
    }
    pub(crate) fn get_text(&self) -> String {
        self.text.clone()
    }
    pub (crate) fn set_text(&mut self, text: String, tag_version: u8) {
        if text.is_ascii() {
            self.text_encoding = TextEncoding::Iso8859_1;
        }else if tag_version == 3 {
            self.text_encoding = TextEncoding::UnicodeUtf16;
        } else if tag_version == 4 {
            self.text_encoding = TextEncoding::UnicodeUtf8
        }
        self.text = text
    }
}

pub(crate) struct UserInfoFrame{
    text_encoding : TextEncoding,
    description : String,
    text : String
}
impl RawSize for UserInfoFrame {
    fn raw_size(&self) -> usize {
        self.raw_bytes().len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.description.to_bytes(&self.text_encoding, true));
        bytes.append(&mut self.text.to_bytes(&self.text_encoding, false));
        // bytes.push(NULL_TERMINATE);
        // bytes.push(NULL_TERMINATE);
        bytes
    }
    
}

pub(crate) struct UrlFrame {
    url : String
}
impl RawSize for UrlFrame {
    fn raw_size(&self) -> usize {
        self.url.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        self.url.clone().into_bytes()
    }
}

pub(crate) struct InvolvedPeopleFrame {
    text_encoding : TextEncoding,
    people_list : String
}
impl RawSize for InvolvedPeopleFrame {
    fn raw_size(&self) -> usize {
        self.raw_bytes().len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.people_list.to_bytes(&self.text_encoding, false));
        bytes
    }


}
pub (crate) struct MusicCdIdframe {
    cd_toc : Vec<u8>
}
impl RawSize for MusicCdIdframe {
    fn raw_size(&self) -> usize {
        self.cd_toc.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.cd_toc.clone()
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

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.time_stamp_format as u8);
        bytes.append(&mut self.raw.clone());
        bytes
    }
}

pub (crate) struct LocationLookupTableFrame {
    raw : Vec<u8>
}

impl RawSize for LocationLookupTableFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
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
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.time_stamp_format as u8);
        bytes.append(&mut self.tempo_data.clone());
        bytes
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
        self.raw_bytes().len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.language.clone().into_bytes());
        bytes.append(&mut self.content_description.to_bytes(&self.text_encoding, true));
        bytes.append(&mut self.text.to_bytes(&self.text_encoding, false));
        bytes
    }
}
impl UnsyncLyricsFrame {
    pub (crate) fn new(lang: String, description: String, text: String) -> Self {
        let text_encoding = if description.is_ascii() && text.is_ascii() {
            TextEncoding::Iso8859_1
        }else {TextEncoding::UnicodeUtf16};
        Self {
            text_encoding,
            language: lang,
            content_description: description,
            text
        }
    }

    pub (crate) fn get_language(&self) -> &String {
        &self.language
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

    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
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
        self.raw_bytes().len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.language.clone().into_bytes());
        bytes.append(&mut self.content_description.to_bytes(&self.text_encoding, true));
        bytes.append(&mut self.text.to_bytes(&self.text_encoding, false));
        //bytes.push(NULL_TERMINATE);
        // bytes.push(NULL_TERMINATE);
        bytes
    }
}
impl CommentFrame {
    pub (crate) fn new(lang: String, description: String, text: String) -> Self {
        let text_encoding = if description.is_ascii() && text.is_ascii() {
            TextEncoding::Iso8859_1
        }else {TextEncoding::UnicodeUtf16};
        Self {
            text_encoding,
            language: lang,
            content_description: description,
            text
        }
    }
    pub (crate) fn get_language(&self) -> &String {
        &self.language
    }
    pub (crate) fn get_description(&self) -> &String {
       &self.content_description 
    }

    pub (crate) fn get_text(&self) -> &String {
        &self.text
     }
}

pub(crate) struct RelativeVolumeAdjustementFrame {
    raw : Vec<u8>
}
impl RawSize for RelativeVolumeAdjustementFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}
pub(crate) struct EqualiisationFrame {
    raw : Vec<u8>
}
impl RawSize for EqualiisationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}
pub(crate) struct ReverbFrame {
    raw : Vec<u8>
}
impl RawSize for ReverbFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
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
        self.raw_bytes().len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encode as u8);
        bytes.append(&mut self.mime_type.to_bytes(&TextEncoding::Iso8859_1, true));
        bytes.push(self.picture_type as u8);
        bytes.append(&mut self.description.to_bytes(&self.text_encode, true));
        bytes.append(&mut self.picture_data.clone());
        bytes
    }
}
impl AttachedPictureFrame {
    pub (crate) fn new(image_format: &PictureFormat, picture_data: &Vec<u8>, picture_type: Option<PictureType>, description: Option<String>) -> Self {
        let description = match description {
            Some(s) => s,
            None => "".to_string()
        };
        let text_encode = if description.is_ascii() {
            TextEncoding::Iso8859_1
        }else {TextEncoding::UnicodeUtf16};

        let picture_type = match picture_type {
            Some(pt) => pt,
            None => PictureType::Other
        };
        Self {
            text_encode,
            mime_type: image_format.to_mime_string(),
            picture_type,
            description,
            picture_data: picture_data.clone()
        }
    }
    pub(crate) fn get_picture_data(&self) -> &Vec<u8>{
        &self.picture_data
    }
}

pub(crate) struct GeneralEncapsulatedObjectFrame {
    text_encoding : TextEncoding,
    mime_type : String,
    filename : String,
    _content : Vec<u8>,
    encapsulated_object : Vec<u8>
}
impl RawSize for GeneralEncapsulatedObjectFrame{
    fn raw_size(&self) -> usize {
        self.raw_bytes().len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.mime_type.to_bytes(&TextEncoding::Iso8859_1, true));
        bytes.append(&mut self.filename.to_bytes(&self.text_encoding, true));
        bytes.push(NULL_TERMINATE);
        if self.text_encoding.is_one_byte(){
            bytes.push(NULL_TERMINATE);
        }
        bytes.append(&mut self.encapsulated_object.clone());
        bytes
    }
}

pub(crate) struct PlayCounterFrame {
    raw : Vec<u8>
}
impl RawSize for PlayCounterFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}
pub(crate) struct PopularimeterFrame {
    raw : Vec<u8>
}
impl RawSize for PopularimeterFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
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
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.buffer_size.to_be_bytes().to_vec());
        bytes.push(self.embedded_info_flag as u8);
        bytes.append(&mut self.offet_next_tag.to_be_bytes().to_vec());
        bytes
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
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.owner_id.to_bytes(&TextEncoding::Iso8859_1, true));
        bytes.append(&mut self.preview_start.to_be_bytes().to_vec());
        bytes.append(&mut self.preview_lenght.to_be_bytes().to_vec());
        bytes.append(&mut self.encryption_info.clone());
        bytes
    }

}
pub (crate) struct LinkedInfoFrame {
    raw : Vec<u8>
}
impl RawSize for LinkedInfoFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}

pub(crate) struct PositionSyncFrame {
    raw : Vec<u8>
}

impl RawSize for PositionSyncFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
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
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.text_encoding as u8);
        bytes.append(&mut self.language.clone().into_bytes());
        bytes.append(&mut self.text.to_bytes(&self.text_encoding, false));
        bytes
    }
}

pub(crate) struct OwnershipFrame {
    raw : Vec<u8>
}
impl RawSize for OwnershipFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}

pub (crate) struct CommercialFrame {
    raw : Vec<u8>
}
impl RawSize for CommercialFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}

pub(crate) struct EncryptionMethodRegistationFrame {
    raw : Vec<u8>
}
impl RawSize for EncryptionMethodRegistationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }

    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}
pub(crate) struct GroupIdentificationRegistationFrame {
    raw : Vec<u8>
}
impl RawSize for GroupIdentificationRegistationFrame{
    fn raw_size(&self) -> usize {
        self.raw.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        self.raw.clone()
    }
}

pub(crate) struct PrivateFrame {
    owner_id : String,
    private_data : Vec<u8>
}

impl RawSize for PrivateFrame {
    fn raw_size(&self) -> usize {
        self.owner_id.len() + 1 /*null terminated bytes*/ + self.private_data.len()
    }
    fn raw_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.owner_id.to_bytes(&TextEncoding::Iso8859_1, true));
        bytes.append(&mut self.private_data.clone());
        bytes
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
            TEXTFRAME(_) => {
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                let string_buff = buffer.drain(0..((size-1) as usize)).collect::<Vec<u8>>();
                // let text = vec_to_string(string_buff, &encode)?;
                let text = string_buff.into_string(&encode)?;
                // println!("Text Value :=> {}", &text);
                let text_frame = TextFrame { text_encoding : encode, text};
                Some( Self::TF( text_frame) )
            }
            TCMP => {
                let string = buffer.drain(0..(size as usize)).collect::<Vec<u8>>().to_utf8()?;
                ////println!("TCMP str : {}", string);
                let is_compilation = string.parse().ok()?;
                Some(
                    Self::ICFF(is_compilation)
                )
            }
            TXXX => {
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                let buffer_i : Vec<u8> = buffer.drain(0..((size-1) as usize)).collect();
                //let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                let strings = buffer_i.split_to_string(&encode);
                let (description, text ) = if strings.len() == 1 {
                    ("".into(), strings.first()?.clone())
                }else {(strings.first()?.clone(), strings.get(1)?.clone() ) };
                // println!("Description :=> {}\nText :=> {}", description, text);
                Some( Self::UIF( UserInfoFrame { text_encoding: encode, description, text}))
            }
            WCOM | WCOP | WOAF | WOAR | WOAS | WORS | WPAY | WPUB => {
                let url =  String::from_utf8(buffer.drain(0..((size) as usize)).collect()).ok()?;
                //println!("buffer remain : {}", buffer.len());
                // println!("url : {}", &url);
                Some( Self::UF( UrlFrame { url } ))  
            },
            COMM => {
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                // println!("encode : {:?}", encode);
                let language = String::from_utf8(buffer.drain(0..3).collect::<Vec<u8>>()).ok()?;
                let buffer_i : Vec<u8> = buffer.drain(0..((size-4) as usize)).collect();
                //let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                let strings = buffer_i.split_to_string(&encode);
                let (content_description, text ) = if strings.len() == 1 {
                    ("".into(), strings.first()?.clone())
                }else {(strings.first()?.clone(), strings.get(1)?.clone() ) };
                // println!("Description :=> {}\nText :=> {}", content_description, text);
                Some(Self::CF(CommentFrame { text_encoding: encode, language, content_description, text }))

            },
            APIC => {
                let start_len = buffer.len();
                let encode = match TextEncoding::from_raw_value(buffer.remove(0)) {
                    Some(e) => e,
                    None => TextEncoding::Iso8859_1,
                };
                // println!("encode : {:?}", encode);
                //let mime_type = first_string(buffer, &encode, true)?;
                let mime_type = buffer.first_matched_string(&encode, true)?;
                let picture_type = PictureType::from_raw_value(buffer.remove(0))?;
                // println!("Mime Type : {}", mime_type);
                //let description = first_string(buffer, &encode, true)?;
                let description = buffer.first_matched_string(&encode, true)?;
                // println!("Description : {}", description);
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
                let people_list = buffer.first_matched_string(&encode, true)?;
                Some( Self::IPF(
                        InvolvedPeopleFrame {
                            text_encoding: encode,
                            people_list }
                    )
                )
            }
            UFID => {
                let start_len = buffer.len();
                //let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let owner_id = buffer.first_matched_string(&TextEncoding::Iso8859_1, true)?;
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
                //println!("encode : {:?}", encode);
                let language = String::from_utf8(buffer.drain(0..3).collect::<Vec<u8>>()).ok()?;
                let buffer_i : Vec<u8> = buffer.drain(0..((size-4) as usize)).collect();
                //let strings = if encode.is_one_byte() {split_to_string_utf8(&buffer_i) } else {split_to_string_utf16(&to_u16_le(&buffer_i))};
                let strings = buffer_i.split_to_string(&encode);
                let (content_description, text ) = if strings.len() == 1 {
                    (strings.first()?.clone(), "".into())
                }else {(strings.first()?.clone(), strings.get(1)?.clone() ) };
                //println!("Description :=> {}\nText :=> {}", content_description, text);
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
                let mime_type = buffer.first_matched_string(&encode, true)?;
                let filename =  buffer.first_matched_string(&encode, true)?;
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
                    _content: content,
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
                let buffer_size = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be().unwrap();
                let embedded_info = buffer.remove(0) == 1;
                let offset = buffer.drain(0..4).collect::<Vec<u8>>().u32_from_be().unwrap();
                Some(Self::RBSF(RecommendedBufferSizeFrame {
                    buffer_size,
                    embedded_info_flag: embedded_info,
                    offet_next_tag: offset
                }))
            }
            AENC => {
                let start_size = buffer.len();
                //let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let owner_id = buffer.first_matched_string(&TextEncoding::Iso8859_1, true)?;
                let preview_start  = buffer.drain(0..2).collect::<Vec<u8>>().u16_from_be()?;
                let preview_lenght  = buffer.drain(0..2).collect::<Vec<u8>>().u16_from_be()?;
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
                //let text = vec_to_string(buffer.drain(0..(size as usize - 4)).collect()
                //, &encode)?;
                let text = buffer.drain(0..(size as usize - 4)).collect::<Vec<u8>>().into_string(&encode)?;
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
                //let owner_id = first_string(buffer, &TextEncoding::UnicodeUtf8, true)?;
                let owner_id = buffer.first_matched_string(&TextEncoding::Iso8859_1, true)?;
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

    fn raw_bytes(&self) -> Vec<u8> {
        match self{
            FrameValue::UFIF(ufif) => ufif.raw_bytes(),
            FrameValue::TF(fv) => fv.raw_bytes(),
            FrameValue::UIF(fv) => fv.raw_bytes(),
            FrameValue::UF(fv) => fv.raw_bytes(),
            FrameValue::IPF(fv) => fv.raw_bytes(),
            FrameValue::MCIF(fv) => fv.raw_bytes(),
            FrameValue::ETF(fv) => fv.raw_bytes(),
            FrameValue::LLTF(fv) => fv.raw_bytes(),
            FrameValue::SYCF(fv) => fv.raw_bytes(),
            FrameValue::ULF(fv) => fv.raw_bytes(),
            FrameValue::SLF(fv) => fv.raw_bytes(),
            FrameValue::CF(fv) => fv.raw_bytes(),
            FrameValue::RVAF(fv) => fv.raw_bytes(),
            FrameValue::EF(fv) => fv.raw_bytes(),
            FrameValue::RF(fv) => fv.raw_bytes(),
            FrameValue::APF(fv) => fv.raw_bytes(),
            FrameValue::GEOF(fv) => fv.raw_bytes(),
            FrameValue::PCF(fv) => fv.raw_bytes(),
            FrameValue::PF(fv) => fv.raw_bytes(),
            FrameValue::RBSF(fv) => fv.raw_bytes(),
            FrameValue::AEF(fv) => fv.raw_bytes(),
            FrameValue::LIF(fv) => fv.raw_bytes(),
            FrameValue::PSF(fv) => fv.raw_bytes(),
            FrameValue::TUF(fv) => fv.raw_bytes(),
            FrameValue::OF(fv) => fv.raw_bytes(),
            FrameValue::CommercialF(fv) => fv.raw_bytes(),
            FrameValue::EMRF(fv) => fv.raw_bytes(),
            FrameValue::GIRF(fv) => fv.raw_bytes(),
            FrameValue::PrivF(fv) => fv.raw_bytes(),
            FrameValue::Undefined(raw) => raw.clone(),
            FrameValue::ICFF(c) => c.to_be_bytes().to_vec(),
            Self::NoValue => vec![],
        }
    }
}


impl FrameValue {
    pub (crate) fn as_attached_picture_frame(&self) -> Option<&AttachedPictureFrame>{
        match self {
            Self::APF(picture_frame) => Some(picture_frame),
            _ => None
        }
    }
    pub (crate) fn as_attached_picture_frame_mut(&mut self) -> Option<&mut AttachedPictureFrame>{
        match self {
            Self::APF(picture_frame) => Some(picture_frame),
            _ => None
        }
    }
    pub (crate) fn as_text_frame(&self) -> Option<&TextFrame>{
        match self {
            Self::TF(tf) => Some(tf),
            _ => None
        }
    }

    pub (crate) fn as_text_frame_mut(&mut self) -> Option<&mut TextFrame>{
        match self {
            Self::TF(tf) => Some(tf),
            _ => None
        }
    }

    pub (crate) fn as_unsynchroned_lyrics_frame(& self) -> Option<&UnsyncLyricsFrame>{
        match self {
            Self::ULF(f) => Some(f),
            _ => None
        }
    }
    pub (crate) fn as_unsynchroned_lyrics_frame_mut(&mut self) -> Option<&mut UnsyncLyricsFrame>{
        match self {
            Self::ULF(f) => Some(f),
            _ => None
        }
    }
    pub(crate) fn as_comment_frame(&self) -> Option<&CommentFrame> {
        match self {
            Self::CF(cf) => Some(cf),
            _ => None 
        }
    }

    pub(crate) fn as_comment_frame_mut(&mut self) -> Option<&mut CommentFrame> {
        match self {
            Self::CF(cf) => Some(cf),
            _ => None 
        }
    }
}