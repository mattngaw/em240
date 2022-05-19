//! Registers and register files.

use std::rc::Rc;
use std::cell::RefCell;
use array_init::*;

/// Stores a single 16-bit value.
pub struct Reg {
    data: Rc<RefCell<u16>>
}

/// Allows for multiple "connections" to a `Reg`
impl Clone for Reg {
    fn clone(&self) -> Self {
        Reg { data: self.data.clone() }
    }
}

impl Reg {
    /// Creates a new `Reg`
    pub fn new() -> Self {
        Reg {
            data: Rc::new(RefCell::new(0x0000u16))
        }
    }

    /// Clears (zeros) the `Reg`
    pub fn clear(&self) -> () {
        self.data.replace(0x0000u16);
    }

    /// Reads from the `Reg`
    pub fn read(&self) -> u16 {
        *self.data.borrow()
    }

    /// Writes to the `Reg`
    pub fn write(&self, data: u16) -> () {
        self.data.replace(data);
    }
}

/// A single-input, dual-output module for reading from and writing to registers
/// (`r0` - `r7`)
/// 
/// r0 is a special register in that it always outputs 0, and can never be 
/// overwritten
pub struct RegFile {
    regs: [Reg; 8],
    sel_rs1: u8,
    sel_rs2: u8,
    sel_rd: u8,
    data: u16
}

impl RegFile {
    /// Creates a new `RegFile`
    pub fn new() -> Self {
        RegFile {
            regs: array_init(|_| Reg::new()),
            sel_rs1: 0u8,
            sel_rs2: 0u8,
            sel_rd: 0u8,
            data: 0u16
        }
    }

    pub fn select(&mut self, sel_rs1: Option<u8>, sel_rs2: Option<u8>,
                             sel_rd: Option<u8>, data: Option<u16>) -> () {
        if sel_rs1.is_some() { self.sel_rs1 = sel_rs1.unwrap() }
        if sel_rs2.is_some() { self.sel_rs2 = sel_rs2.unwrap() }
        if sel_rd.is_some() { self.sel_rd = sel_rd.unwrap() }
        if data.is_some() { self.data = data.unwrap() }
    }

    /// Reads from two registers given `rs1` and `rs2`
    pub fn read(&self) -> (u16, u16) {
        debug_assert!(self.sel_rs1 < 8);
        debug_assert!(self.sel_rs2 < 8);
        (self.regs[self.sel_rs1 as usize].read(), 
         self.regs[self.sel_rs2 as usize].read())
    }

    /// Writes to a single register `rd`
    pub fn write(&mut self) -> () {
        debug_assert!(self.sel_rd < 8);
        if self.sel_rd != 0u8 {
            self.regs[self.sel_rd as usize].write(self.data);
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
        regfile.select(Some(0), Some(1), Some(0), Some(0x1234u16));
        assert_eq!(regfile.read(), (0x0000, 0x0000));
        regfile.write();
        regfile.select(None, Some(3), Some(3), None);
        assert_eq!(regfile.read(), (0x0000, 0x0000));
        regfile.write();
        regfile.select(Some(2), None, None, None);
        assert_eq!(regfile.read(), (0x0000, 0x1234));
    }
}