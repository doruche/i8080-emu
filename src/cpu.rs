#![allow(unused)]

use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::fmt::DebugStruct;

use crate::dram::Dram;
use crate::error::Error;
use crate::instruction::{Instruction, RegPair, Src};
use crate::utils::*;

pub const RAM_SIZE: usize = 65536;

const CARRY_BIT: u8 = 0;
const PARITY_BIT: u8 = 2;
const AUXILIARY_CARRY_BIT: u8 = 4;
const ZERO_BIT: u8 = 6;
const SIGN_BIT: u8 = 7;

#[derive(Debug)]
pub struct Cpu {
    a: u8,  // accumulator

    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    sp: u16,
    pc: u16,

    ram: Dram,
    flag: u8,
    inte: bool,
}

// interface
impl Cpu {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            sp: RAM_SIZE as u16,
            pc: 0x40, // 0x00 ~ 0x3f for rst instructions.
            ram: Dram::new(),
            flag: 2, // 0bsz0c0p1c
            inte: false, 
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.pc as usize >= RAM_SIZE {
                break Ok(());
            }
            let instruc = self.fetch()?;
            self.excecute(instruc)?;
        }
    }
}

// utils
impl Cpu {
    fn fetch(&mut self) -> Result<Instruction, Error> {
        use Instruction::*;

        let first_byte = self.ram.load_byte(self.pc);
        self.pc += 1;

        match first_byte {
            0 => Ok(NOP),

            0b00111111 => Ok(CMC),
            0b00110111 => Ok(STC),

            _ if bitmatch(first_byte, 0b00000100, 0b11000111) => 
            Ok(INR(idx2src((first_byte & 0b00111000) >> 3))),
            _ if bitmatch(first_byte, 0b00000101, 0b11000111) =>
            Ok(DCR(idx2src((first_byte & 0b00111000) >> 3))),
            
            0b00101111 => Ok(CMA),
            0b00100111 => Ok(DAA),

            _ if bitmatch(first_byte, 0b01000000, 0b11000000) => {
                let dst = idx2src((first_byte & 0b00111000) >> 3);
                let src = idx2src(first_byte & 0b00000111);
                Ok(MOV(dst, src))
            },
            _ if bitmatch(first_byte, 0b00000010, 0b11100111) => {
                let pair = idx2rp(first_byte & 0b00010000);
                if bittest(first_byte, 3) {
                    Ok(LDAX(pair))
                } else {
                    Ok(SATX(pair))
                }
            },

            _ if bitmatch(first_byte, 0b10000000, 0b11000000) => {
                let op = (first_byte & 0b00111000) >> 3;
                let reg = idx2src(first_byte & 0b00000111);
                Ok(match op {
                    0 => ADD(reg),
                    1 => ADC(reg),
                    2 => SUB(reg),
                    3 => SBB(reg),
                    4 => ANA(reg),
                    5 => XRA(reg),
                    6 => ORA(reg),
                    7 => CMP(reg),
                    _ => unreachable!(),
                })
            },


            0b00000111 => Ok(RLC),
            0b00001111 => Ok(RRC),
            0b00010111 => Ok(RAL),
            0b00011111 => Ok(RAR),

            _ if bitmatch(first_byte, 0b11000101, 0b11001111) => {
                let rp = idx2rp((first_byte & 0b00110000) >> 4);
                Ok(PUSH(rp))
            },
            _ if bitmatch(first_byte, 0b11000001, 0b11001111) => {
                let rp = idx2rp((first_byte & 0b00110000) >> 4);
                Ok(POP(rp))
            },
            _ if bitmatch(first_byte, 0b00001001, 0b11001111) => {
                let rp = idx2rp((first_byte & 0b00110000) >> 4);
                Ok(DAD(rp))
            },
            _ if bitmatch(first_byte, 0b00000011, 0b11001111) => {
                let rp = idx2rp((first_byte & 0b00110000) >> 4);
                Ok(INX(rp))
            }, 
            _ if bitmatch(first_byte, 0b00001011, 0b11001111) => {
                let rp = idx2rp((first_byte & 0b00110000) >> 4);
                Ok(DCX(rp))
            },
            _ if bitmatch(first_byte, 0b11101011, 255) =>
            Ok(XCHG),
            _ if bitmatch(first_byte, 0b11100011, 255) =>
            Ok(XTHL),
            _ if bitmatch(first_byte, 0b11111001, 255) =>
            Ok(SPHL),

            _ if bitmatch(first_byte, 0b00000001, 0b11001111) => {
                let low_data = self.next_byte();
                let high_data = self.next_byte();
                let rp = (first_byte & 0b00110000) >> 4;
                Ok(LXI(rp, low_data, high_data))
            }
            _ if bitmatch(first_byte, 0b00000110, 0b11000111) => {
                let reg = (first_byte & 0b00111000) >> 4;
                let data = self.next_byte();
                Ok(MVI(reg, data))
            }

            0b11000110 => Ok(ADI(self.next_byte())),
            0b11001110 => Ok(ACI(self.next_byte())),
            0b11010110 => Ok(SUI(self.next_byte())),
            0b11011110 => Ok(SBI(self.next_byte())),
            0b11100110 => Ok(ANI(self.next_byte())),
            0b11101110 => Ok(XRI(self.next_byte())),
            0b11110110 => Ok(ORI(self.next_byte())),
            0b11111110 => Ok(CPI(self.next_byte())),

            0b00110010 => Ok(STA(self.next_byte(), self.next_byte())),
            0b00111010 => Ok(LDA(self.next_byte(), self.next_byte())),
            0b00100010 => Ok(SHLD(self.next_byte(), self.next_byte())),
            0b00101010 => Ok(LHLD(self.next_byte(), self.next_byte())),

            0b11101001 => Ok(PCHL),
            0b11000011 => Ok(JMP(self.next_byte(), self.next_byte())),
            0b11011010 => Ok(JC(self.next_byte(), self.next_byte())),
            0b11010010 => Ok(JNC(self.next_byte(), self.next_byte())),
            0b11001010 => Ok(JZ(self.next_byte(), self.next_byte())),
            0b11000010 => Ok(JNZ(self.next_byte(), self.next_byte())),
            0b11111010 => Ok(JM(self.next_byte(), self.next_byte())),
            0b11110010 => Ok(JP(self.next_byte(), self.next_byte())),
            0b11101010 => Ok(JPE(self.next_byte(), self.next_byte())),
            0b11100010 => Ok(JPO(self.next_byte(), self.next_byte())),

            0b11001101 => Ok(CALL(self.next_byte(), self.next_byte())),
            0b11011100 => Ok(CC(self.next_byte(), self.next_byte())),
            0b11010100 => Ok(CNC(self.next_byte(), self.next_byte())),
            0b11001100 => Ok(CZ(self.next_byte(), self.next_byte())),
            0b11000100 => Ok(CNZ(self.next_byte(), self.next_byte())),
            0b11111100 => Ok(CM(self.next_byte(), self.next_byte())),
            0b11110100 => Ok(CP(self.next_byte(), self.next_byte())),
            0b11101100 => Ok(CPE(self.next_byte(), self.next_byte())),
            0b11100100 => Ok(CPO(self.next_byte(), self.next_byte())),

            0b11001001 => Ok(RET),
            0b11011000 => Ok(RC),
            0b11010000 => Ok(RNC),
            0b11001000 => Ok(RZ),
            0b11000000 => Ok(RNZ),
            0b11111000 => Ok(RM),
            0b11110000 => Ok(RP),
            0b11101000 => Ok(RPE),
            0b11100000 => Ok(RPO),

            0b11111011 => Ok(EI),
            0b11110011 => Ok(DI),

            0b11011011 => Ok(IN(self.next_byte())),
            0b11010011 => Ok(OUT(self.next_byte())),

            0b01110110 => Ok(HLT),

            _ => Err(Error::UnknownOpcode(first_byte))
        }
    }

    fn excecute(&mut self, instruction: Instruction) -> Result<(), Error> {
        use Instruction::*;
        
        match instruction {
            NOP => (),
            CMC => self.flag ^= 1 << CARRY_BIT,
            STC => self.flag |= 1 << CARRY_BIT,
            INR(src) => {
                let src = self.get_src(src);
                let (res, _, parity, auxiliary_carry, zero, sign) = flagged_add(*src, 1);
                *src = res;
                self.set_flags(None, Some(parity), Some(auxiliary_carry), Some(zero), Some(sign));
            },
            DCR(src) => {
                let src = self.get_src(src);
                let (res, _, parity, auxiliary_carry, zero, sign) = flagged_sub(*src, 1);
                *src = res;
                self.set_flags(None, Some(parity), Some(auxiliary_carry), Some(zero), Some(sign));
            },
            CMA => self.a = !self.a,
            DAA => {
                let mut a: u8 = 0;
                let mut c = self.get_flag(CARRY_BIT);
                let lsb = self.a & 0x0f;
                let msb = self.a >> 4;
                // If the least significant four bits of the accumulator represents a number greater than 9, or if the Auxiliary
                // Carry bit is equal to one, the accumulator is incremented by six. Otherwise, no incrementing occurs.
                if (lsb > 9) || self.get_flag(AUXILIARY_CARRY_BIT) {
                    a += 0x06;
                }
                // If the most significant four bits of the accumulator now represent a number greater than 9, or if the normal
                // carry bit is equal to one, the most sign ificant four bits of the accumulator are incremented by six.
                if (msb > 9) || self.get_flag(CARRY_BIT) || (msb >= 9 && lsb > 9) {
                    a += 0x60;
                    c = true;
                }

                let (res, carry, parity, aux, zero, sign) = flagged_add(self.a, self.a);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));               

                self.set_flag(CARRY_BIT, c);
            },
            MOV(dst, src) => {
                let src = *self.get_src(src);
                let dst = self.get_src(dst);
                *dst = src;
            },
            SATX(rp) => {
                match rp {
                    RegPair::BC => self.ram.save_byte(self.get_bc_addr(), self.a),
                    RegPair::DE => self.ram.save_byte(self.get_de_addr(), self.a),
                    _ => unreachable!(),
                };
            },
            LDAX(rp) => {
                match rp {
                    RegPair::BC => self.a = self.ram.load_byte(self.get_bc_addr()),
                    RegPair::DE => self.a = self.ram.load_byte(self.get_de_addr()),
                    _ => unreachable!(),
                };
            },
            ADD(reg) => {
                let src = *self.get_src(reg);
                let (res, carry, parity, aux, zero, sign) = flagged_add(self.a, src);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            ADC(reg) => {
                let c = if self.get_flag(CARRY_BIT) {1} else {0};
                let src = *self.get_src(reg);
                let (res, carry, parity, aux, zero, sign) = flagged_add(self.a, src.wrapping_add(c));
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            SUB(reg) => {
                let src = *self.get_src(reg);
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, src);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            SBB(reg) => {
                let c = if self.get_flag(CARRY_BIT) {1} else {0};
                let src = *self.get_src(reg);
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, src.wrapping_add(c));
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            ANA(reg) => {
                self.a &= *self.get_src(reg);
                self.set_logical_flag();
            },
            XRA(reg) => {
                self.a ^= *self.get_src(reg);
                self.set_logical_flag();
            },
            ORA(reg) => {
                self.a |= *self.get_src(reg);
                self.set_logical_flag();
            },
            CMP(reg) => {
                let a = self.a;
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, *self.get_src(reg));
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
                self.a = a;
            },
            RLC => {
                let c = bittest(self.a, 7);
                self.a = (self.a << 1) | u8::from(c);
                self.set_flag(CARRY_BIT, c);
            },
            RRC => {
                let c = bittest(self.a, 0);
                self.a = if c { 0x80 | (self.a >> 1) } else { self.a >> 1 };
                self.set_flag(CARRY_BIT, c);
            },
            RAL => {
                let h = bittest(self.a, 7);
                let c = self.get_flag(CARRY_BIT);
                self.set_flag(CARRY_BIT, h);
                self.a = (self.a << 1) | u8::from(c);
            },
            RAR => {
                let l = bittest(self.a, 0);
                let c = self.get_flag(CARRY_BIT);
                self.set_flag(CARRY_BIT, l);
                self.a = if c { 0x80 | (self.a >> 1) } else { self.a >> 1 };
            },
            PUSH(rp) => {
                self.sp -= 2;
                let (b1, b2) = match rp {
                    RegPair::BC => (self.b, self.c),
                    RegPair::DE => (self.d, self.e),
                    RegPair::HL => (self.h, self.l),
                    RegPair::PSW => (self.a, self.flag),
                };
                self.ram.save_byte(self.sp + 1, b1);
                self.ram.save_byte(self.sp, b2);
            },
            POP(rp) => {
                let (src1, src2) = match rp {
                    RegPair::BC => (&mut self.b, &mut self.c),
                    RegPair::DE => (&mut self.d, &mut self.e),
                    RegPair::HL => (&mut self.h, &mut self.l),
                    RegPair::PSW => (&mut self.a, &mut self.flag),
                };
                *src1 = self.ram.load_byte(self.sp + 1);
                *src2 = self.ram.load_byte(self.sp);
                self.sp += 2;
            },
            DAD(rp) => {
                let x = self.get_rp_val(rp);
                let (res, carry) = x.overflowing_add(get_u16(self.h, self.l));
                self.set_flag(CARRY_BIT, carry);
            },
            INX(rp) => {
                let x = self.get_rp_val(rp).wrapping_add(1);
                match rp {
                    RegPair::BC => (self.b, self.c) = split_u16(x),
                    RegPair::DE => (self.d, self.e) = split_u16(x),
                    RegPair::HL => (self.h, self.l) = split_u16(x),
                    RegPair::PSW => (self.a, self.flag) = split_u16(x),
                };
            },
            DCX(rp) => {
                let x = self.get_rp_val(rp).wrapping_sub(1);
                match rp {
                    RegPair::BC => (self.b, self.c) = split_u16(x),
                    RegPair::DE => (self.d, self.e) = split_u16(x),
                    RegPair::HL => (self.h, self.l) = split_u16(x),
                    RegPair::PSW => (self.a, self.flag) = split_u16(x),
                };
            },
            XCHG => {
                (self.d, self.h) = (self.h, self.d);
                (self.e, self.l) = (self.l, self.d);
            },
            XTHL => {
                let (h, l) = (self.h, self.l);
                (self.h, self.l) = split_u16(self.ram.load_word(self.sp));
                self.ram.save_word(self.sp, get_u16(h, l));
            },
            SPHL => self.ram.save_word(self.sp, get_u16(self.h, self.l)),
            _ => ()
        };

        Ok(())
    }

    fn get_src(&mut self, src: Src) -> &mut u8 {
        match src {
            Src::B => &mut self.b,
            Src::C => &mut self.c,
            Src::D => &mut self.d,
            Src::E => &mut self.e,
            Src::H => &mut self.h,
            Src::L => &mut self.l,
            Src::A => &mut self.a,
            Src::Mem => self.ram.get_ptr(self.get_hl_addr())
        }
    }


    fn set_logical_flag(&mut self) {
        self.set_flag(CARRY_BIT, false);
        self.set_flag(PARITY_BIT, self.a.count_ones() % 2 == 0);
        self.set_flag(AUXILIARY_CARRY_BIT, false);
        self.set_flag(ZERO_BIT, self.a == 0);
        self.set_flag(SIGN_BIT, self.a > 0x80);
    }

    fn set_flags(&mut self, carry: Option<bool>, parity: Option<bool>, aux: Option<bool>, zero: Option<bool>, sign: Option<bool>) {
        carry.map(|carry| if carry { self.flag &= 1 << CARRY_BIT });
        parity.map(|parity| if parity { self.flag &= 1 << PARITY_BIT });
        aux.map(|auxiliary_carry| if auxiliary_carry { self.flag &= 1 << AUXILIARY_CARRY_BIT });
        zero.map(|zero| if zero { self.flag &= 1 << ZERO_BIT });
        sign.map(|sign| if sign { self.flag &= 1 << SIGN_BIT });
    }

    fn set_flag(&mut self, bit: u8, flag: bool) {
        let b = if flag {1} else {0};
        self.flag &= b << bit;
    }

    fn get_flag(&self, bit: u8) -> bool {
        (self.flag & (1 << bit)) >> bit == 1
    }

    fn get_hl_addr(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    fn get_bc_addr(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    fn get_de_addr(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    fn get_rp_val(&self, rp: RegPair) -> u16 {
        match rp {
            RegPair::BC => get_u16(self.b, self.c),
            RegPair::DE => get_u16(self.d, self.e),
            RegPair::HL => get_u16(self.h, self.l),
            RegPair::PSW => get_u16(self.a, self.flag),
        }
    }

    fn next_byte(&mut self) -> u8 {
        self.pc += 1;
        self.ram.load_byte(self.pc - 1)
    }
}