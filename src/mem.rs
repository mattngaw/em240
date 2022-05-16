use crate::reg::Reg;

pub struct Mem {
    data: [u8; 0x10000],
    mar: Reg,
    mdr: Reg
}

impl Mem {
    pub fn new(mar: Reg, mdr: Reg) -> Self {
        Mem {
            data: [0x00; 0x10000],
            mar: mar,
            mdr: mdr
        }
    }

    pub fn read(&mut self) -> () {
        let addr = self.mar.read() & !(1u16);
        let upper_addr = addr;
        let lower_addr = addr + 1;
        let upper = self.data[upper_addr as usize] as u16;
        let lower = self.data[lower_addr as usize] as u16;
        self.mdr.write((upper << 8) | (lower));
    }

    pub fn write(&mut self) -> () {
        let addr = self.mar.read() & !(1u16);
        let upper_addr = addr;
        let lower_addr = addr + 1;
        let upper_data = ((self.mdr.read() & 0xFF00u16) >> 8) as u8;
        let lower_data = (self.mdr.read() & 0x00FFu16) as u8;
        self.data[upper_addr as usize] = upper_data;
        self.data[lower_addr as usize] = lower_data;
    }
}

mod tests {
    use super::Mem;
    use crate::reg::Reg;

    #[test]
    fn test_mem_new() {
        let mar = Reg::new();
        let mdr = Reg::new();
        let mem = Mem::new(mar, mdr);
        assert_eq!(mem.data, [0x00u8; 0x10000]);
    }

    #[test]
    fn test_mem_read() {
        let mar = Reg::new();
        let mdr = Reg::new();
        let mut mem = Mem::new(mar.clone(), mdr.clone());
        mem.read();
        assert_eq!(mdr.read(), 0x0000);
        mar.write(0x0001u16);
        mem.read();
        assert_eq!(mdr.read(), 0x0000);
    }

    #[test]
    fn test_mem_write() {
        let mar = Reg::new();
        let mdr = Reg::new();
        mdr.write(0x0102u16);
        let mut mem = Mem::new(mar.clone(), mdr.clone());
        mem.write();
        assert_eq!(mem.data[0x0000 as usize], 0x01);
        assert_eq!(mem.data[0x0001 as usize], 0x02);
    }

    #[test]
    fn test_mem_read_and_write() {
        let mar = Reg::new();
        let mdr = Reg::new();
        mdr.write(0x0102u16);
        let mut mem = Mem::new(mar.clone(), mdr.clone());
        mem.write();
        mem.read();
        assert_eq!(mdr.read(), 0x0102);
        mar.write(0xFFFEu16);
        mdr.write(0x1234);
        mem.write();
        mem.read();
        assert_eq!(mdr.read(), 0x1234);
    }
}