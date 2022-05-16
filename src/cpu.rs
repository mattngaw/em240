//! The heart of the RISC240 ISA

use crate::reg::*;
use crate::mem::*;

/// Broken-down interpretaiton of a RISC240 instruction
pub struct Instr {
    pub opcode: u8, // The actual type of instruction
    pub rd: u8, // Destination register
    pub rs1: u8, // Source register 1
    pub rs2: u8 // Source register 2
}

impl Instr {
    const OPCODE_MASK: u16 = 0xFD00;
    const RD_MASK: u16 = 0x01C0;
    const RS1_MASK: u16 = 0x0038;
    const RS2_MASK: u16 = 0x0007;

    /// Forms an `Instr` from a 16-bit instruction
    pub fn new(word: u16) -> Self {
        Instr {
            opcode: ((word & Self::OPCODE_MASK) >> 9) as u8,
            rd: ((word & Self::RD_MASK) >> 6) as u8,
            rs1: ((word & Self::RS1_MASK) >> 6) as u8,
            rs2: ((word & Self::RS2_MASK) >> 3) as u8
        }
    }
}

/// Components of RISC240 processor
pub struct CPU {
    pub pc: Reg, // Program counter
    pub cc: u8, // Condition codes
    pub rf: RegFile, // Register file
    pub ir: Reg, // Instruction register
    pub mem: Mem, // Memory
    pub mar: Reg, // Memory-address register
    pub mdr: Reg, // Memory-data register
}

impl CPU {
    pub fn new() -> Self {
        let mar = Reg::new();
        let mdr = Reg::new();
        CPU { 
            pc: Reg::new(),
            cc: 0b0000u8,
            rf: RegFile::new(),
            ir: Reg::new(),
            mar: mar.clone(),
            mdr: mdr.clone(),
            mem: Mem::new(mar.clone(), mdr.clone()),
        }
    }

    pub fn init(&mut self) {
        
    }

    fn fetch(&mut self) -> () {
        self.mar.write(self.pc.read());
        self.mem.read();
        self.ir.write(self.mdr.read());
    }

    fn decode(&self) -> Instr {
        Instr::new(self.ir.read())
    }

    fn execute(&mut self, instr: Instr) -> ()  {
        match instr.opcode {
            0x00 => { // ADD rd, rs1, rs2
                let (rs1, rs2) = self.rf.read(instr.rs1, instr.rs2);
                self.rf.write(rs1 + rs2, instr.rd);
            }
            0x18 => { // ADDI rd, rs1, imm | LI rd, imm
                let (rs1, _) = self.rf.read(instr.rs1, instr.rs2);
                self.pc.write(self.pc.read() + 2);
                self.mar.write(self.pc.read());
                self.mem.read();
                self.rf.write(rs1 + self.mdr.read(), instr.rd);
            }
            0x48 => { // AND rd, rs1, rs2
                let (rs1, rs2) = self.rf.read(instr.rs1, instr.rs2);
                self.rf.write(rs1 & rs2, instr.rd);
            }
            0x7C => { // BRA addr

            }
            0x54 => { // BRC addr

            }
            0x4C => { // BRN addr

            }
            0x6C => { // BRNZ addr

            }
            0x5C => { // BRV addr

            }
            0x64 => { // BRZ addr

            }
            0x14 => { // LW rd, rs1, imm

            }
            0x10 => { // MV rd, rs1

            }
            0x40 => { // NOT rd, rs1

            }
            0x50 => { // OR rd, rs1, rs2

            }
            0x60 => { // SLL rd, rs1, rs2

            }
            0x61 => { // SLLI rd, rs1, shamt

            }
            0x28 => { // SLT rd, rs1, rs2

            }
            0x29 => { // SLTI rd, rs1, imm

            }
            0x78 => { // SRA rd, rs1, rs2

            }
            0x79 => { // SRAI rd, rs1, shamt

            }
            0x70 => { // SRL rd, rs1, rs2

            }
            0x71 => { // SRLI rd, rs1, shamt

            }
            0x7F => { // STOP

            }
            0x08 => { // SUB rd, rs1, rs2

            }
            0x1C => { // SW rs1, rs2, imm

            }
            0x58 => { // XOR rd, rs1, rs2

            }
            _ => ()
        }
        self.pc.write(self.pc.read() + 2);
    }

    pub fn run(&mut self) {
        self.pc.clear();

        loop {
            self.fetch();
            let instr = self.decode();
            self.execute(instr);
        }
    }
}