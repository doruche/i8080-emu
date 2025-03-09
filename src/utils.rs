#![allow(unused)]

use crate::instruction::{RegPair, Src};

pub fn bitmatch(bits: u8, pattern: u8, mask: u8) -> bool {
    (bits & mask) == (pattern & mask)
}

pub fn bittest(bits: u8, n: u8) -> bool {
    (bits & (1 << n)) != 0
}

// Carry, parity, auxiliary carry, zero, sign
pub fn flagged_add(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    let (res, carry) = x.overflowing_add(y);
    (res,
    carry,
    res % 2 == 0,
    (x & 0xf) + (y & 0xf) > 0xf,
    res == 0,
    (res & 0b10000000) >> 7 == 1)
}
pub fn flagged_sub(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    flagged_add(x, !y + 1)
}

pub fn idx2src(idx: u8) -> Src {
    match idx {
        0 => Src::B,
        1 => Src::C,
        2 => Src::D,
        3 => Src::E,
        4 => Src::H,
        5 => Src::L,
        6 => Src::Mem,
        7 => Src::A,
        _ => unreachable!(),
    }
}

pub fn idx2rp(idx: u8) -> RegPair {
    match idx {
        0 => RegPair::BC,
        1 => RegPair::DE,
        2 => RegPair::HL,
        3 => RegPair::PSW,
        _ => unreachable!(),
    }
}

pub fn get_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn split_u16(val: u16) -> (u8, u8) {
    ((val >> 8) as u8, (val & 0xff) as u8)
}