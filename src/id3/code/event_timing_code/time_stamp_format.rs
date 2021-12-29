
#[repr(u8)]
#[derive(Clone, Copy)]
pub(crate) enum TimeStampFormat {
    MPEGFrames = 0x01,
    Milliseconds = 0x02
}

impl TimeStampFormat {
    pub (crate) fn from_raw_value(value : u8) -> Option<Self>{
        match value {
            1 => Some(Self::MPEGFrames),
            2 => Some(Self::Milliseconds),
            _ => None
        }
    }
}