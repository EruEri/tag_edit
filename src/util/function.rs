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





