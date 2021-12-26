#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum PictureType {
    Other = 0x00,
    FileIcon,
    OtherFileIcon,
    CoverFront,
    CoverBlack,
    LeafletPage,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    MovieScrenCapture,
    BrightColouredFish,
    Illustration,
    ArtistLogotype,
    PublisherLogotype
}

impl PictureType {
    pub(crate) fn from_raw_value(value : u8) -> Option<Self>{
        match value {
            0 => Some(Self::Other),
            1 => Some (Self::FileIcon),
            2 => Some (Self::OtherFileIcon),
            3 => Some (Self::CoverFront),
            4 => Some (Self::CoverBlack),
            5 => Some (Self::LeafletPage),
            6 => Some (Self::Media),
            7 => Some (Self::LeadArtist),
            8 => Some (Self::Artist),
            9 => Some (Self::Conductor),
            10 => Some (Self::Band),
            11 => Some (Self::Composer),
            12 => Some (Self::Lyricist),
            13 => Some (Self::RecordingLocation),
            14 => Some (Self::DuringRecording),
            15 => Some (Self::DuringPerformance),
            16 => Some (Self::MovieScrenCapture),
            17 => Some (Self::BrightColouredFish),
            18 => Some (Self::Illustration),
            19 => Some (Self::ArtistLogotype),
            20 => Some (Self::PublisherLogotype),
            _ => None
        }
    }
}