#![allow(unused)]

use std::io;
use std::io::prelude::*;
use std::fs::File;

use cpu::Cpu;

mod cpu;
mod dram;
mod device;
mod instruction;
mod clock_cycles;
mod error;
mod utils;
mod test_instr;

fn main() {
    let mut args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: i8080 [image-file]");
        return;
    };

    let mut f;
    match File::open(&args[1]) {
        Ok(file) => f = file,
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        },
    };
    let mut image = Vec::new();
    f.read_to_end(&mut image).unwrap();

    let mut cpu = Cpu::new().load(&image);
    cpu.test().unwrap();
}
