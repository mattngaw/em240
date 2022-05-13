pub struct Mem {
    pub data: [u8; 0x10000]
}

impl Mem {
    pub fn new() -> Self {
        Mem {
            data: [0x00; 0x10000]
        }
    }

    pub fn read(&self, addr: u16) -> u16 {
        let addr = addr & !(1u16);
        let upper_addr = addr;
        let lower_addr = addr + 1;
        let upper = self.data[upper_addr as usize] as u16;
        let lower = self.data[lower_addr as usize] as u16;
        (upper << 8) | (lower)
    }

    pub fn write(&mut self, addr: u16, data: u16) -> () {
        let addr = addr & !(1u16);
        let upper_addr = addr;
        let lower_addr = addr + 1;
        let upper_data = ((data & 0xFF00) >> 8) as u8;
        let lower_data = (data & 0x00FF) as u8;
        self.data[upper_addr as usize] = upper_data;
        self.data[lower_addr as usize] = lower_data;
    }
}

mod tests {
    use super::Mem;

    #[test]
    fn test_mem_new() {
        let mem = Mem::new();
        assert_eq!(mem.data, [0x00u8; 0x10000]);
    }

    #[test]
    fn test_mem_read() {
        let mem = Mem::new();
        assert_eq!(mem.read(0x0000u16), 0x0000);
        assert_eq!(mem.read(0x0001u16), 0x0000);
    }

    #[test]
    fn test_mem_write() {
        let mut mem = Mem::new();
        mem.write(0x0000, 0x0102);
        assert_eq!(mem.data[0x0000 as usize], 0x01);
        assert_eq!(mem.data[0x0001 as usize], 0x02);
    }

    #[test]
    fn test_mem_read_and_write() {
        let mut mem = Mem::new();
        mem.write(0x0000, 0x0102);
        assert_eq!(mem.read(0x0000u16), 0x0102);
        assert_eq!(mem.read(0x0001u16), 0x0102);
        assert_eq!(mem.read(0x0002u16), 0x0000);
        assert_eq!(mem.read(0x0003u16), 0x0000);
        mem.write(0xFFFF, 0x1234);
        assert_eq!(mem.read(0xFFFE), 0x1234);
    }
}