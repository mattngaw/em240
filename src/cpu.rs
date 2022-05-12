use crate::reg::RegFile;
use crate::mem::*;
use crate::alu::*;
use crate::opcodes::*;

pub struct CPU {
    pub pc: u16,
    pub cc: u8,
    pub rf: RegFile,
    pub ir: u8
}

impl CPU {
    pub fn new() -> Self {
        CPU { 
            pc: 0x0000,
            cc: 0b0000,
            rf: RegFile::new(),
            ir: 0x00
        }
    }

    pub fn run(&mut self, ) {

    }

    fn fetch() {}

    fn decode() {}

    fn execute() {}
}