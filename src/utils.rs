#![allow(unused)]

pub fn bitmatch(bits: u8, pattern: u8, mask: u8) -> bool {
    (bits & mask) == (pattern & mask)
}

// Carry, parity, auxiliary carry, zero, sign
pub fn flagged_add(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    let (tx, ty) = (x & 0b1111, y & 0b1111);
    let auxiliary_carry = (tx + ty) & 0b10000 == 1;
    

    let (res, carry) = x.overflowing_add(y);
    (res,
    carry,
    res % 2 == 0,
    auxiliary_carry,
    res == 0,
    (res & 0b10000000) >> 7 == 1)
}
pub fn flagged_sub(x: u8, y: u8) -> (u8, bool, bool, bool, bool, bool) {
    flagged_add(x, !y + 1)
}