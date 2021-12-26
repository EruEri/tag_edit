#[derive(Copy, Clone)]
pub enum AudioFormat {
    MP3,
    FLAC,
    OTHER
}

#[derive(Clone, Copy)]
pub enum PictureFormat {
    PNG,
    JPEG
}

impl PictureFormat {
    pub (crate) fn to_mime_string(&self) -> String {
        match self {
            PictureFormat::PNG => "image/png".into(),
            PictureFormat::JPEG => "image/jpeg".into(),
        }
    }
}