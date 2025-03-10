#![allow(unused)]

use std::arch::x86_64::_SIDD_CMP_EQUAL_ANY;
use std::fmt::DebugStruct;
use std::time;

use crate::device::Device;
use crate::dram::Dram;
use crate::error::Error;
use crate::instruction::{Instruction, RegPair, Src};
use crate::utils::*;

pub const RAM_SIZE: usize = 65536;

const CLOCK_RATE: u32 = 2_000_000; // 2.0MHz
const STEP_TIME: u32 = 16;
const STEP_CYCLES: u32 = (STEP_TIME as f64 / (1000_f64 / CLOCK_RATE as f64)) as u32;
const PORT_NUM: usize = 256;   // i8080 adopts PMIO.

pub const CARRY_BIT: u8 = 0;
pub const PARITY_BIT: u8 = 2;
pub const AUXILIARY_CARRY_BIT: u8 = 4;
pub const ZERO_BIT: u8 = 6;
pub const SIGN_BIT: u8 = 7;

pub struct Cpu {
    pub a: u8,  // accumulator

    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,
    pub halted: bool,
    pub flag: u8,
    pub inte: bool,
    pub step_cycles: u32,
    pub step_zero: time::SystemTime,
    pub ram: Dram,
    pub devices: [Option<Box<dyn Device>>; PORT_NUM],
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
            pc: 0, // 0x00 ~ 0x3f for rst instructions.
            halted: false,
            step_cycles: 0,
            step_zero: time::SystemTime::now(),
            ram: Dram::new(),
            devices: [const { None }; PORT_NUM],
            flag: 2, // 0bsz0c0p1c
            inte: false, 
        }
    }

    pub fn load(mut self, data: &[u8]) -> Self {
        self.ram.load_slice(data);
        self
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.halted {
                return Ok(());
            }
            if self.pc as usize >= RAM_SIZE {
                return Err(Error::PcOutofRange);
            }
            let instruc = self.fetch()?;
            self.excecute(instruc)?;
        }
    }

    pub fn next(&mut self) -> Result<(), Error> {
        let ins = self.fetch()?;
        self.excecute(ins)?;
        Ok(())
    }

    pub fn test(&mut self) -> Result<(), Error> {
        println!("*******************");
        self.ram.save_byte(0x0005, 0xc9);
        // Because tests used the pseudo instruction ORG 0x0100
        self.pc = 0x0100;
        loop {
            if self.halted {
                break Ok(());
            }
            self.next()?;
            if self.pc == 0x05 {
                if self.c == 0x09 {
                    let mut a = self.get_de_addr();
                    loop {
                        let c = self.ram.load_byte(a);
                        if c as char == '$' {
                            break;
                        } else {
                            a += 1;
                        }
                        print!("{}", c as char);
                    }
                }
                if self.c == 0x02 {
                    print!("{}", self.e as char);
                }
            }
            if self.pc == 0x00 {
                println!("");
                println!("");
                break Ok(());
            }
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
                let pair = idx2rp_psw((first_byte & 0b00010000) >> 4);
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
                let rp = idx2rp_psw((first_byte & 0b00110000) >> 4);
                Ok(PUSH(rp))
            },
            _ if bitmatch(first_byte, 0b11000001, 0b11001111) => {
                let rp = idx2rp_psw((first_byte & 0b00110000) >> 4);
                Ok(POP(rp))
            },
            _ if bitmatch(first_byte, 0b00001001, 0b11001111) => {
                let rp = idx2rp_sp((first_byte & 0b00110000) >> 4);
                Ok(DAD(rp))
            },
            _ if bitmatch(first_byte, 0b00000011, 0b11001111) => {
                let rp = idx2rp_sp((first_byte & 0b00110000) >> 4);
                Ok(INX(rp))
            }, 
            _ if bitmatch(first_byte, 0b00001011, 0b11001111) => {
                let rp = idx2rp_sp((first_byte & 0b00110000) >> 4);
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
                Ok(LXI(idx2rp_sp(rp), low_data, high_data))
            },
            _ if bitmatch(first_byte, 0b00000110, 0b11000111) => {
                let reg = (first_byte & 0b00111000) >> 3;
                let data = self.next_byte();
                Ok(MVI(idx2src(reg), data))
            },

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

            _ if bitmatch(first_byte, 0b11000111, 0b11000111) => {
                let exp = (first_byte & 0b00111000) >> 3;
                Ok(RST(exp))
            },
            0b11111011 => Ok(EI),
            0b11110011 => Ok(DI),

            0b11011011 => Ok(IN(self.next_byte())),
            0b11010011 => Ok(OUT(self.next_byte())),

            0b01110110 => Ok(HLT),

            _ => {
                println!("unknown ins");
                Err(Error::UnknownOpcode(first_byte))
            }
        }
    }

    fn excecute(&mut self, instruction: Instruction) -> Result<(), Error> {
        use Instruction::*;
        
        //println!(
        //    "{:?} PC={:04x} SP={:04x} A={:02x} F={:02x} B={:02x} C={:02x} D={:02x} E={:02x} H={:02x} L={:02x} flag={:08b}",
        //    &instruction,
        //    self.pc,
        //    self.sp,
        //    self.a,
        //    self.flag,
        //    self.b,
        //    self.c,
        //    self.d,
        //    self.e,
        //    self.h,
        //    self.l,
        //    self.flag,
        //);

        match instruction {
            NOP => (),
            CMC => self.set_flag(CARRY_BIT, !self.get_flag(CARRY_BIT)),
            STC => self.set_flag(CARRY_BIT, true),
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

                let (res, carry, parity, aux, zero, sign) = flagged_add(self.a, a);
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
                let a = self.a;
                let c = u8::from(self.get_flag(CARRY_BIT));
                let src = (*self.get_src(reg));
                let (res, _, parity, _, zero, sign) = flagged_add(self.a, src.wrapping_add(c));
                self.a = res;
                self.set_flags(Some(u16::from(a) + u16::from(c) + u16::from(src) > 0xff), 
                Some(parity), Some((a & 0xf) + (src & 0xf) + c > 0xf), 
                Some(zero), Some(sign));
            },
            SUB(reg) => {
                let src = *self.get_src(reg);
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, src);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            SBB(reg) => {
                let a = self.a;
                let c = u8::from(self.get_flag(CARRY_BIT));
                let src = *self.get_src(reg);
                let (res, _, parity, _, zero, sign) = flagged_sub(self.a, src.wrapping_add(c));
                self.a = res;
                self.set_flags(Some(u16::from(a) < u16::from(src) + u16::from(c)),
                Some(parity), Some((a as i8 & 0x0f) - (src as i8 & 0x0f) - (c as i8) >= 0x00),
                Some(zero), Some(sign));
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
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, *self.get_src(reg));
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
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
                let (b1, b2) = match rp {
                    RegPair::BC => (self.b, self.c),
                    RegPair::DE => (self.d, self.e),
                    RegPair::HL => (self.h, self.l),
                    RegPair::PSW => (self.a, self.flag),
                    _ => unreachable!(),
                };
                self.push(get_u16(b1, b2));
            },
            POP(rp) => {
                let (hi, lo) = split_u16(self.pop());
                let (src1, src2) = match rp {
                    RegPair::BC => (&mut self.b, &mut self.c),
                    RegPair::DE => (&mut self.d, &mut self.e),
                    RegPair::HL => (&mut self.h, &mut self.l),
                    RegPair::PSW => (&mut self.a, &mut self.flag),
                    _ => unreachable!(),
                };
                *src1 = hi;
                *src2 = lo;
            },
            DAD(rp) => {
                let x = self.get_rp_val(rp);
                let (res, carry) = x.overflowing_add(get_u16(self.h, self.l));
                (self.h, self.l) = split_u16(res);
                self.set_flag(CARRY_BIT, carry);
            },
            INX(rp) => {
                let x = self.get_rp_val(rp).wrapping_add(1);
                match rp {
                    RegPair::BC => (self.b, self.c) = split_u16(x),
                    RegPair::DE => (self.d, self.e) = split_u16(x),
                    RegPair::HL => (self.h, self.l) = split_u16(x),
                    RegPair::SP => self.sp = self.sp.wrapping_add(1),
                    _ => unreachable!(),
                };
            },
            DCX(rp) => {
                let x = self.get_rp_val(rp).wrapping_sub(1);
                match rp {
                    RegPair::BC => (self.b, self.c) = split_u16(x),
                    RegPair::DE => (self.d, self.e) = split_u16(x),
                    RegPair::HL => (self.h, self.l) = split_u16(x),
                    RegPair::SP => self.sp = self.sp.wrapping_sub(1),
                    _ => unreachable!(),
                };
            },
            XCHG => {
                (self.d, self.h) = (self.h, self.d);
                (self.e, self.l) = (self.l, self.e);
            },
            XTHL => {
                let (h, l) = (self.h, self.l);
                (self.h, self.l) = split_u16(self.ram.load_word(self.sp));
                self.ram.save_word(self.sp, get_u16(h, l));
            },
            SPHL => self.sp = get_u16(self.h, self.l),

            LXI(rp, low_data, high_data) => {
                match rp {
                    RegPair::BC => (self.b, self.c) = (high_data, low_data),
                    RegPair::DE => (self.d, self.e) = (high_data, low_data),
                    RegPair::HL => (self.h, self.l) = (high_data, low_data),
                    RegPair::SP => self.ram.save_word(self.sp, get_u16(high_data, low_data)),
                    _ => unreachable!(),
                };
            },
            MVI(src, data) => *self.get_src(src) = data,
            ADI(data) => {
                let (res, carry, parity, aux, zero, sign) = flagged_add(self.a, data);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));               
            },
            ACI(data) => {
                let (res, carry, parity, aux, zero, sign) = 
                flagged_add(self.a, data.wrapping_add(u8::from(self.get_flag(CARRY_BIT))));
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            SUI(data) => {
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, data);
                self.a = res;
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },
            SBI(data) => {
                let a = self.a;
                let c = u8::from(self.get_flag(CARRY_BIT));
                let (res, _, parity, _, zero, sign) = flagged_sub(self.a, data.wrapping_add(c));
                self.a = res;
                self.set_flags(Some(u16::from(a) < u16::from(data) + u16::from(c)),
                Some(parity), Some((a as i8 & 0x0f) - (data as i8 & 0x0f) - (c as i8) >= 0x00),
                Some(zero), Some(sign));
            },
            ANI(data) => {
                self.a &= data;
                self.set_logical_flag();
            },
            XRI(data) => {
                self.a ^= data;
                self.set_logical_flag();
            },
            ORI(data) => {
                self.a |= data;
                self.set_logical_flag();
            },
            CPI(data) => {
                let (res, carry, parity, aux, zero, sign) = flagged_sub(self.a, data);
                self.set_flags(Some(carry), Some(parity), Some(aux), Some(zero), Some(sign));
            },

            STA(low_add, hi_add) => self.ram.save_byte(get_u16(hi_add, low_add), self.a),
            LDA(low_add, hi_add) => self.a = self.ram.load_byte(get_u16(hi_add, low_add)),
            SHLD(low_add, hi_add) => self.ram.save_word(get_u16(hi_add, low_add), get_u16(self.h, self.l)),
            LHLD(low_add, hi_add) =>(self.h, self.l) = split_u16(self.ram.load_word(get_u16(hi_add, low_add))),

            PCHL => self.pc = get_u16(self.h, self.l),
            JMP(low_add, hi_add) => self.pc = get_u16(hi_add, low_add),
            JC(low_add, hi_add) => if self.get_flag(CARRY_BIT) { self.pc = get_u16(hi_add, low_add) },
            JNC(low_add, hi_add) => if !self.get_flag(CARRY_BIT) { self.pc = get_u16(hi_add, low_add) },
            JZ(low_add, hi_add) => if self.get_flag(ZERO_BIT) { self.pc = get_u16(hi_add, low_add) },
            JNZ(low_add, hi_add) => if !self.get_flag(ZERO_BIT) {  self.pc = get_u16(hi_add, low_add) },
            JM(low_add, hi_add) => if self.get_flag(SIGN_BIT) { self.pc = get_u16(hi_add, low_add) },
            JP(low_add, hi_add) => if !self.get_flag(SIGN_BIT) { self.pc = get_u16(hi_add, low_add) },
            JPE(low_add, hi_add) => if self.get_flag(PARITY_BIT) { self.pc = get_u16(hi_add, low_add) },
            JPO(low_add, hi_add) => if !self.get_flag(PARITY_BIT) { self.pc = get_u16(hi_add, low_add) },

            CALL(low_add, hi_add) => {
                self.push(self.pc);
                self.pc = get_u16(hi_add, low_add);
            },
            CC(low_add, hi_add) => {
                if self.get_flag(CARRY_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },
            CNC(low_add, hi_add) => {
                if !self.get_flag(CARRY_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },    
            CZ(low_add, hi_add) => {
                if self.get_flag(ZERO_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },  
            CNZ(low_add, hi_add) => {
                if !self.get_flag(ZERO_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },  
            CM(low_add, hi_add) => {
                if self.get_flag(SIGN_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },  
            CP(low_add, hi_add) => {
                if !self.get_flag(SIGN_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },
            CPE(low_add, hi_add) => {
                if self.get_flag(PARITY_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },
            CPO(low_add, hi_add) => {
                if !self.get_flag(PARITY_BIT) {
                    self.push(self.pc);
                    self.pc = get_u16(hi_add, low_add);
                }
            },

            RET => self.pc = self.pop(),
            RC => if self.get_flag(CARRY_BIT) { self.pc = self.pop() },
            RNC => if !self.get_flag(CARRY_BIT) { self.pc = self.pop() },
            RZ => if self.get_flag(ZERO_BIT) { self.pc = self.pop() },
            RNZ => if !self.get_flag(ZERO_BIT) { self.pc = self.pop() },
            RM => if self.get_flag(SIGN_BIT) { self.pc = self.pop() },
            RP => if !self.get_flag(SIGN_BIT) { self.pc = self.pop() },
            RPE => if self.get_flag(PARITY_BIT) { self.pc = self.pop() },
            RPO => if !self.get_flag(PARITY_BIT) { self.pc = self.pop() },

            RST(exp) => {
                self.push(self.pc);
                self.pc = exp.wrapping_mul(8) as u16;
            },
            EI => self.inte = true,
            DI => self.inte = false,

            IN(device_no) => {
                if let Some(device) = &mut self.devices[device_no as usize] {
                    self.a = device.read();
                } else {
                    eprintln!("No such device.");
                    self.halted = true;
                }
            },
            OUT(device_no) => {
                if let Some(device) = &mut self.devices[device_no as usize] {
                    device.write(self.a);
                } else {
                    eprintln!("No such device.");
                    self.halted = true;
                }
            },

            HLT => self.halted = true,
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

    fn push(&mut self, word: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.ram.save_word(self.sp, word);
    }

    fn pop(&mut self) -> u16 {
        self.sp = self.sp.wrapping_add(2);
        self.ram.load_word(self.sp.wrapping_sub(2))
    }

    fn set_logical_flag(&mut self) {
        self.set_flag(CARRY_BIT, false);
        self.set_flag(PARITY_BIT, self.a.count_ones() % 2 == 0);
        self.set_flag(AUXILIARY_CARRY_BIT, false);
        self.set_flag(ZERO_BIT, self.a == 0);
        self.set_flag(SIGN_BIT, self.a > 0x80);
    }

    fn set_flags(&mut self, carry: Option<bool>, parity: Option<bool>, aux: Option<bool>, zero: Option<bool>, sign: Option<bool>) {
        carry.map(|carry| bitset(&mut self.flag, CARRY_BIT, carry));
        parity.map(|parity| bitset(&mut self.flag, PARITY_BIT, parity));
        aux.map(|aux| bitset(&mut self.flag, AUXILIARY_CARRY_BIT, aux));
        zero.map(|zero| bitset(&mut self.flag, ZERO_BIT, zero));
        sign.map(|sign| bitset(&mut self.flag, SIGN_BIT, sign));
    }

    pub(crate) fn set_flag(&mut self, bit: u8, flag: bool) {
        bitset(&mut self.flag, bit, flag);
    }

    pub(crate) fn get_flag(&self, bit: u8) -> bool {
        let v = (self.flag & (1 << bit)) >> bit;
        v == 1
    }

    pub(crate) fn get_hl_addr(&self) -> u16 {
        get_u16(self.h, self.l)
    }

    fn get_bc_addr(&self) -> u16 {
        get_u16(self.b, self.c)
    }

    fn get_de_addr(&self) -> u16 {
        get_u16(self.d, self.e)
    }

    fn get_rp_val(&self, rp: RegPair) -> u16 {
        match rp {
            RegPair::BC => get_u16(self.b, self.c),
            RegPair::DE => get_u16(self.d, self.e),
            RegPair::HL => get_u16(self.h, self.l),
            RegPair::PSW => get_u16(self.a, self.flag),
            RegPair::SP => self.sp,
        }
    }

    fn next_byte(&mut self) -> u8 {
        self.pc = self.pc.wrapping_add(1);
        self.ram.load_byte(self.pc - 1)
    }
}