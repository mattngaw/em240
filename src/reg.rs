use std::rc::Rc;
use std::cell::RefCell;
use array_init::*;

pub struct Reg {
    data: Rc<RefCell<u16>>
}

impl Clone for Reg {
    fn clone(&self) -> Reg {
        Reg { data: self.data.clone() }
    }
}

impl Reg {
    pub fn new() -> Self {
        Reg {
            data: Rc::new(RefCell::new(0x0000u16))
        }
    }

    pub fn clear(&self) -> () {
        self.data.replace(0x0000u16);
    }

    pub fn read(&self) -> u16 {
        *self.data.borrow()
    }

    pub fn write(&self, data: u16) -> () {
        self.data.replace(data);
    }
}

pub struct RegFile {
    pub regs: [Reg; 8],
}

impl RegFile {
    pub fn new() -> Self {
        RegFile {
            regs: array_init(|_| Reg::new())
        }
    }

    pub fn read(&self, sel_rs1: u8, sel_rs2: u8) -> (u16, u16) {
        debug_assert!(sel_rs1 < 8);
        debug_assert!(sel_rs2 < 8);
        (self.regs[sel_rs1 as usize].read(), self.regs[sel_rs2 as usize].read())
    }

    pub fn write(&mut self, data: u16, sel_rd: u8) -> () {
        debug_assert!(sel_rd < 8);
        if sel_rd != 0u8 {
            self.regs[sel_rd as usize].write(data);
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_reg() {
        let reg = Reg::new();
        assert_eq!(reg.read(), 0x0000u16);
        reg.write(0x1234u16);
        assert_eq!(reg.read(), 0x1234u16);
        reg.clear();
        assert_eq!(reg.read(), 0x0000u16);
    }

    #[test]
    fn test_regfile() {
        let mut regfile = RegFile::new();
        assert_eq!(regfile.read(0u8, 1u8), (0x0000, 0x0000));
        regfile.write(0x1234u16, 0u8);
        assert_eq!(regfile.read(0u8, 3u8), (0x0000, 0x0000));
        regfile.write(0x1234u16, 3u8);
        assert_eq!(regfile.read(2u8, 3u8), (0x0000, 0x1234));
    }
}