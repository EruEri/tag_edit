
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum TextContent {
    Other = 0x00,
    Lyrics,
    TextTranscription,
    MovementPartName,
    Events,
    Chord,
    Trivial
}

impl TextContent {
    pub(crate) fn from_raw_value(value : u8) -> Option<Self> {
        match value {
            0 => Some(Self::Other),
            1 => Some(Self::Lyrics),
            2 => Some(Self::TextTranscription),
            3 => Some(Self::MovementPartName),
            4 => Some(Self::Events),
            5 => Some(Self::Chord),
            6 => Some(Self::Trivial),
            _ => None
        }
    }
}