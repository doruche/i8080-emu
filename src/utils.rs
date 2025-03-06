#![allow(unused)]

pub fn bitmatch(bits: u8, pattern: u8, mask: u8) -> bool {
    (bits & mask) == (pattern & mask)
}