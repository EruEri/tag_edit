#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AudioFormat {
    MP3,
    FLAC,
    OTHER
}

/// Picture file format
#[derive(Clone)]
pub enum PictureFormat {
    PNG,
    JPEG,
    OTHER(String)
}

impl PictureFormat {
    pub (crate) fn to_mime_string(&self) -> String {
        match self {
            PictureFormat::PNG => "image/png".into(),
            PictureFormat::JPEG => "image/jpeg".into(),
            Self::OTHER(s) => format!("image/{}", s)
        }
    }
}