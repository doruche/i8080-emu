#![allow(unused)]

use std::{fs::FileTimes, io::repeat};

use crate::{error::Error, instruction::Instruction, utils::bitmatch};

const RAM_SIZE: usize = 65536;

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

    ram: [u8; RAM_SIZE],
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
            sp: 0,
            pc: 0,
            ram: [0; RAM_SIZE],
            flag: 0,
            inte: true,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        todo!()
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            if self.pc as usize >= self.ram.len() {
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

        let first_byte = self.ram[self.pc as usize];
        self.pc += 1;

        match first_byte {
            0 => Ok(NOP),

            0b00111111 => Ok(CMC),
            0b00110111 => Ok(STC),

            _ if bitmatch(first_byte, 0b00000100, 0b11000111) => 
            Ok(INR((first_byte & 0b00111000) >> 3)),
            _ if bitmatch(first_byte, 0b00000101, 0b11000111) =>
            Ok(DCR((first_byte & 0b00111000) >> 3)),
            
            0b00101111 => Ok(CMA),
            0b00100111 => Ok(DAA),

            _ if bitmatch(first_byte, 0b01000000, 0b11000000) => {
                let dst = first_byte & 0b00111000;
                let src = first_byte & 0b00000111;
                Ok(MOV(dst, src))
            },
            _ if bitmatch(first_byte, 0b00000010, 0b11100111) => {
                let pair = first_byte & 0b00010000;
                let kind = first_byte & 0b00001000;
                if kind == 0 {
                    Ok(SATX(pair))
                } else {
                    Ok(LDAX(pair))
                }
            },

            _ if bitmatch(first_byte, 0b10000000, 0b11000000) => {
                let op = (first_byte & 0b00111000) >> 3;
                let reg = first_byte & 0b00000111;
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
                let rp = (first_byte & 0b00110000) >> 4;
                Ok(PUSH(rp))
            },
            _ if bitmatch(first_byte, 0b11000001, 0b11001111) => {
                let rp = (first_byte & 0b00110000) >> 4;
                Ok(POP(rp))
            },
            _ if bitmatch(first_byte, 0b00001001, 0b11001111) => {
                let rp = (first_byte & 0b00110000) >> 4;
                Ok(DAD(rp))
            },
            _ if bitmatch(first_byte, 0b00000011, 0b11001111) => {
                let rp = (first_byte & 0b00110000) >> 4;
                Ok(INX(rp))
            }, 
            _ if bitmatch(first_byte, 0b00001011, 0b11001111) => {
                let rp = (first_byte & 0b00110000) >> 4;
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
            _ => ()
        };

        Ok(())
    }

    fn idx2reg(&self, idx: u8) -> u8 {
        match idx {
            0 => self.a,
            1 => self.b,
            2 => self.c,
            3 => self.d,
            4 => self.e,
            5 => self.h,
            7 => self.l,
            _ => unreachable!(),
        }
    }

    fn next_byte(&mut self) -> u8 {
        self.pc += 1;
        self.ram[self.pc as usize - 1]
    }
}