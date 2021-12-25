pub (crate) const LSBYTE_MASK : u16 = 0x00FF;

pub (crate) fn unsynchsafe(input : u32) -> u32 {
    let mut out : u32 = 0;
    let mut mask : u32 = 0x7F000000;

    while mask != 0 {
        out >>= 1;
        out |= input & mask;
        mask >>= 8;
    }
    out
}

pub (crate) fn synchsafe(input : u32) -> u32 {
    let mut input_copie = input;
    let mut out : u32 = 0x7F;
    let mut mask : u32 = out;

    while (mask ^ 0x7FFFFFFF) != 0 {
        out = input_copie & !mask;
        out <<= 1;
        out |= input_copie & mask;
        mask = ((mask + 1) << 8) - 1;
        input_copie = out;
    }
    out
}


pub trait ToU32 {
    fn to_u32_be(&self) -> Option<u32>;
}
pub trait ToU16 {
    fn to_u16_be(&self) -> Option<u16>;
}

impl ToU32 for Vec<u8> {
    fn to_u32_be(&self) -> Option<u32> {
        if self.len() < 3 {
            None
        }else {
            Some(
                u32::from_be_bytes([self.get(0).unwrap().clone(), self.get(1).unwrap().clone(),
                self.get(2).unwrap().clone(), self.get(3).unwrap().clone()
                ])
            )
        }
    }
}

impl ToU16 for Vec<u8> {
    fn to_u16_be(&self) -> Option<u16> {
        if self.len() < 1 {
            None
        }else {
            Some(
                u16::from_be_bytes([self.get(0).unwrap().clone(), self.get(1).unwrap().clone()])
            )
        }
    }
}
