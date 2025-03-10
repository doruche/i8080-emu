#![allow(unused)]

use crate::instruction::{RegPair, Src};

pub fn bitmatch(bits: u8, pattern: u8, mask: u8) -> bool {
    (bits & mask) == (pattern & mask)
}

pub fn bittest(bits: u8, n: u8) -> bool {
    (bits & (1 << n)) != 0
}

pub fn bitset(bits: &mut u8, whence: u8, set: bool) {
    let mask = 1 << whence;
    if set {
        *bits |= mask;
    } else {
        *bits &= !mask;
    }
}

// Carry, parity, auxiliary carry, zero, sign
pub fn flagged_add(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    let (res, carry) = x.overflowing_add(y);
    (res,
    carry,
    res.count_ones() % 2 == 0,
    (x & 0xf) + (y & 0xf) > 0xf,
    res == 0,
    bittest(res, 7))
}
pub fn flagged_sub(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    let (res, carry) = x.overflowing_sub(y);
    (res,
    x < y,
    res.count_ones() % 2 == 0,
    (x as i8 & 0xf) - (y as i8 & 0xf) >= 0x00,
    res == 0,
    bittest(res, 7))
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

pub fn idx2rp_psw(idx: u8) -> RegPair {
    match idx {
        0 => RegPair::BC,
        1 => RegPair::DE,
        2 => RegPair::HL,
        3 => RegPair::PSW,
        _ => unreachable!(),
    }
}

pub fn idx2rp_sp(idx: u8) -> RegPair {
    match idx {
        0 => RegPair::BC,
        1 => RegPair::DE,
        2 => RegPair::HL,
        3 => RegPair::SP,
        _ => unreachable!(),
    }
}

pub fn get_u16(high: u8, low: u8) -> u16 {
    ((high as u16) << 8) | (low as u16)
}

pub fn split_u16(val: u16) -> (u8, u8) {
    ((val >> 8) as u8, (val & 0xff) as u8)
}