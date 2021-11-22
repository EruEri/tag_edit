#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum ID3HeaderFLAG {
    Unsynchronisation = 0b10_000_000,
    ExtendedHeader = 0b01_000_000,
    ExperimentalIndicator = 0b00_100_000
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum ID3FRAMEHEADERFLAGSB1 {
    TagAlterPreservation = 0b10_000_000,
    FileAlterPreservation = 0b01_000_000,
    ReadOnly = 0b00_100_000
}
#[repr(u8)]
#[derive(Copy, Clone)]
pub(crate) enum ID3FRAMEHEADERFLAGSB2 {
    Compression = 0b10_000_000,
    Encryption = 0b01_000_000,
    GroupingIdentity = 0b00_100_000
}
