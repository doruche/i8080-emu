#![allow(unused)]

pub trait Device {
    fn read(&mut self) -> u8;
    fn write(&mut self, byte: u8);
}