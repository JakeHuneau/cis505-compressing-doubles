// Bit represents a 0 or 1
#[derive(Debug, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
}

impl Bit {
    pub fn to_u64(&self) -> u64 {
        match self {
            Bit::Zero => 0,
            Bit::One => 1,
        }
    }
}

// If in Gorilla or Sprintz
#[derive(PartialEq)]
pub enum CompressionMode {
    Gorilla = 0,
    Sprintz = 1,
}

// If in encode or decode mode
#[derive(PartialEq)]
pub enum Mode {
    Encode = 0,
    Decode = 1,
}
