pub enum FrameValue {
    Int(i32),
    Str(String),
    RawValue(Vec<u8>)
}


impl FrameValue {
    pub fn is_int(&self) -> bool {
        match self{
            FrameValue::Int(_) => true,
            _ => false
        }
    }
    pub fn is_str(&self) -> bool {
        match self{
            FrameValue::Str(_) => true,
            _ => false
        }
    }

    pub fn is_raw_value(&self) -> bool {
        match self{
            FrameValue::RawValue(_) => true,
            _ => false
        }
    }
}