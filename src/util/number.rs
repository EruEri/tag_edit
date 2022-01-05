#[allow(non_camel_case_types)]
pub (crate) struct u24 {
    n : u32
}

impl u24 {
    pub (crate) fn from_be_bytes(s : [u8; 3]) -> Self {
        Self {
            n : u32::from_be_bytes([
                00, s[0], s[1], s[2]
            ])
        }
    }

    pub (crate) fn to_be_bytes(&self) -> [u8; 3] {
        let bytes = self.n.to_be_bytes();
       [bytes[1], bytes[2], bytes[3]]
    }

    pub (crate) fn value(&self) -> u32 {
        self.n
    }
}

impl From<u32> for u24 {
    fn from(n: u32) -> Self {
        Self {
            n
        }
    }
}