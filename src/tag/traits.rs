pub(crate) trait TagSize {
    fn size(&self) -> u32;
}

pub(super) trait FrameSize {
    fn total_size(&self) -> u32;
}

pub(crate) trait RawSize {
    /// Return the footprint of the struct
    /// For the Vec<Items> :
    /// raw_size = sizeof Item * Vector's lenght
    fn raw_size(&self) -> usize;
}