#![allow(unused)]

use crate::{cpu::RAM_SIZE, utils::get_u16};

#[derive(Debug)]
pub struct Dram {
    memory: [u8; RAM_SIZE],
}

impl Dram {
    pub fn new() -> Self {
        Self {
            memory: [0; RAM_SIZE],
        }
    }
    
    pub fn get_ptr(&mut self, addr: u16) -> &mut u8 {
        &mut self.memory[addr as usize]
    }

    pub fn load_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn load_word(&self, addr: u16) -> u16 {
        get_u16(self.memory[(addr + 1) as usize], self.memory[addr as usize])
    }

    pub fn save_byte(&mut self, addr: u16, byte: u8) {
        self.memory[addr as usize] = byte;
    }

    pub fn save_word(&mut self, addr: u16, word: u16) {
        self.memory[addr as usize] = (word & 0xff) as u8;
        self.memory[addr as usize + 1] = (word >> 8) as u8;
    }
}