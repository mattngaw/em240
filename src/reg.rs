pub struct RegFile {
    pub regs: [u16; 8],
}

impl RegFile {
    pub fn new() -> Self {
        RegFile {
            regs: [0x0000; 8]
        }
    }

    pub fn read(&self, sel_rs1: u8, sel_rs2: u8) -> (u16, u16) {
        debug_assert!(sel_rs1 < 8);
        debug_assert!(sel_rs2 < 8);
        (self.regs[sel_rs1 as usize], self.regs[sel_rs1 as usize])
    }

    pub fn write(&mut self, data: u16, sel_rd: u8) -> () {
        debug_assert!(sel_rd < 8);
        if sel_rd != 0 {
            self.regs[sel_rd as usize] = data;
        }
    }
}