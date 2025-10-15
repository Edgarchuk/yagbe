pub struct Memory {
    bank0: [u8; 0x4000],
    bank1: [u8; 0x4000],
    vram: [u8; 0x2000],
    eram: [u8; 0x2000],
    wram: [u8; 0x1000],
    wram_extra: [u8; 0x1000],
    oam: [u8; 0x00A0],
    io: [u8; 0x0080],
    hram: [u8; 0x07F],
    ie: u8,
}

#[derive(Debug)]
pub enum MemoryError {
    RomBigSize,
    Checksum8NotEq,
    Checksum16NotEq,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            bank0: [0u8; 0x4000],
            bank1: [0u8; 0x4000],
            vram: [0u8; 0x2000],
            eram: [0u8; 0x2000],
            wram: [0u8; 0x1000],
            wram_extra: [0u8; 0x1000],
            oam: [0u8; 0x00A0],
            io: [0u8; 0x80],
            hram: [0u8; 0x07F],
            ie: 0u8,
        }
    }
    pub fn with_rom(rom: &[u8]) -> Result<Self, MemoryError> {
        if rom.len() > 0x8000 {
            return Err(MemoryError::RomBigSize);
        }
        let mut bank0 = [0u8; 0x4000];
        let mut bank1 = [0u8; 0x4000];
        bank0.copy_from_slice(&rom[..0x4000]);
        bank1.copy_from_slice(&rom[0x4000..]);
        let memory = Self {
            bank0,
            bank1,
            vram: [0u8; 0x2000],
            eram: [0u8; 0x2000],
            wram: [0u8; 0x1000],
            wram_extra: [0u8; 0x1000],
            oam: [0u8; 0x00A0],
            io: [0u8; 0x80],
            hram: [0u8; 0x07F],
            ie: 0u8,
        };
        Ok(memory)
    }

    pub fn check_logo(&self) -> bool {
        let logo_org = include_bytes!("..\\assets\\nintendo.bin");
        println!("{:?}", logo_org);
        println!("{:?}", &self.bank1[0x0104..0x0134]);
        logo_org == &self.bank1[0x0104..0x0134]
    }

    pub fn check_checksum(&self) -> Result<(), MemoryError> {
        let mut checksum = 0u8;
        for idx in 0x0134u16..0x014Du16 {
            checksum = checksum.wrapping_sub(self.read_u8(idx)).wrapping_sub(1);
        }
        println!("{} {}", checksum, self.read_u8(0x014Du16));
        if checksum != self.read_u8(0x014Du16) {
            return Err(MemoryError::Checksum8NotEq);
        }
        Ok(())
    }

    pub fn read_u8(&self, address: u16) -> u8 {
        let idx = address as usize;
        match address {
            ..0x4000 => self.bank0[idx],
            0x4000..0x8000 => self.bank1[idx - 0x4000],
            0x8000..0xA000 => self.vram[idx - 0x8000],
            0xA000..0xC000 => self.eram[idx - 0xA000],
            0xC000..0xD000 => self.wram[idx - 0xC000],
            0xD000..0xE000 => self.wram_extra[idx - 0xD000],
            0xE000..0xFE00 => self.wram[idx - 0xE000],
            0xFE00..0xFEA0 => self.oam[idx - 0xFE00],
            0xFEA0..0xFF00 => 0u8,
            0xFF00..0xFF80 => self.io[idx - 0xFF00],
            0xFF80..0xFFFF => self.hram[idx - 0xFF80],
            0xFFFF => self.ie,
        }
    }

    pub fn write_u8(&mut self, address: u16, value: u8) {
        let idx = address as usize;
        match address {
            ..0x4000 => self.bank0[idx] = value,
            0x4000..0x8000 => self.bank1[idx - 0x4000] = value,
            0x8000..0xA000 => self.vram[idx - 0x8000] = value,
            0xA000..0xC000 => self.eram[idx - 0xA000] = value,
            0xC000..0xD000 => self.wram[idx - 0xC000] = value,
            0xD000..0xE000 => self.wram_extra[idx - 0xD000] = value,
            0xE000..0xFE00 => self.wram[idx - 0xE000] = value,
            0xFE00..0xFEA0 => self.oam[idx - 0xFE00] = value,
            0xFEA0..0xFF00 => (),
            0xFF00..0xFF80 => self.io[idx - 0xFF00] = value,
            0xFF80..0xFFFF => self.hram[idx - 0xFF80] = value,
            0xFFFF => self.ie = value,
        }
    }

    pub fn read_u16(&self, address: u16) -> u16 {
        (self.read_u8(address) as u16) + ((self.read_u8(address + 1) as u16) << 8)
    }

    pub fn write_u16(&mut self, address: u16, val: u16) {
        self.write_u8(address, (val & 0xFF) as u8);
        self.write_u8(address + 1, (val >> 8) as u8);
    }
}

pub struct Cpu {
    pub memory: Memory,
    pub register: Register,
    cycle: u64,
}

#[derive(Default)]
pub struct Register {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterU8Label {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterU16Label {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Register {
    fn new() -> Self {
        Self {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100,
        }
    }

    pub fn read_u8(&self, reg: RegisterU8Label) -> u8 {
        match reg {
            RegisterU8Label::A => self.a,
            RegisterU8Label::F => self.f,
            RegisterU8Label::B => self.b,
            RegisterU8Label::C => self.c,
            RegisterU8Label::D => self.d,
            RegisterU8Label::E => self.e,
            RegisterU8Label::H => self.h,
            RegisterU8Label::L => self.l,
        }
    }

    pub fn write_u8(&mut self, reg: RegisterU8Label, val: u8) {
        match reg {
            RegisterU8Label::A => self.a = val,
            RegisterU8Label::F => self.f = val,
            RegisterU8Label::B => self.b = val,
            RegisterU8Label::C => self.c = val,
            RegisterU8Label::D => self.d = val,
            RegisterU8Label::E => self.e = val,
            RegisterU8Label::H => self.h = val,
            RegisterU8Label::L => self.l = val,
        }
    }

    pub fn read_u16(&self, reg: RegisterU16Label) -> u16 {
        match reg {
            RegisterU16Label::AF => ((self.a as u16) << 8) + self.f as u16,
            RegisterU16Label::BC => ((self.b as u16) << 8) + self.c as u16,
            RegisterU16Label::DE => ((self.d as u16) << 8) + self.e as u16,
            RegisterU16Label::HL => ((self.h as u16) << 8) + self.l as u16,
            RegisterU16Label::SP => self.sp,
            RegisterU16Label::PC => self.pc,
        }
    }

    pub fn write_u16(&mut self, reg: RegisterU16Label, val: u16) {
        match reg {
            RegisterU16Label::AF => {
                self.a = (val >> 8) as u8;
                self.f = val as u8
            }
            RegisterU16Label::BC => {
                self.b = (val >> 8) as u8;
                self.c = val as u8
            }
            RegisterU16Label::DE => {
                self.d = (val >> 8) as u8;
                self.e = val as u8
            }
            RegisterU16Label::HL => {
                self.h = (val >> 8) as u8;
                self.l = val as u8
            }
            RegisterU16Label::SP => self.sp = val,
            RegisterU16Label::PC => self.pc = val,
        }
    }

    pub fn inc_u16(&mut self, reg: RegisterU16Label) {
        self.write_u16(reg, self.read_u16(reg) + 1);
    }

    pub fn dec_u16(&mut self, reg: RegisterU16Label) {
        self.write_u16(reg, self.read_u16(reg) - 1);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressType {
    RegisterU8(RegisterU8Label),
    RegisterU18(RegisterU16Label),
    RegisterU18Inc(RegisterU16Label),
    RegisterU18Dec(RegisterU16Label),
    ValueU16,
}

#[derive(Debug, Clone, Copy)]
pub enum Argument {
    RegisterU8(RegisterU8Label),
    RegisterU16(RegisterU16Label),
    ValueU8,
    ValueU16,
    Address(AddressType),
    AdjustedStackPointer,
}

impl Cpu {
    pub fn new(memory: Memory) -> Self {
        Self {
            memory: memory,
            register: Register::new(),
            cycle: 0,
        }
    }

    pub fn load_next_u8(&mut self) -> u8 {
        let result = self.memory.read_u8(self.register.pc);
        self.register.pc += 1;
        result
    }

    pub fn load_next_u16(&mut self) -> u16 {
        (self.load_next_u8() as u16) + ((self.load_next_u8() as u16) << 8)
    }

    pub fn get_address_by(&mut self, address: AddressType) -> u16 {
        match address {
            AddressType::RegisterU8(reg) => 0xFF00 + self.register.read_u8(reg) as u16,
            AddressType::RegisterU18(reg) => self.register.read_u16(reg),
            AddressType::RegisterU18Inc(reg) => {
                let res = self.register.read_u16(reg);
                self.register.inc_u16(reg);

                res
            }
            AddressType::RegisterU18Dec(reg) => {
                let res = self.register.read_u16(reg);
                self.register.dec_u16(reg);

                res
            }
            AddressType::ValueU16 => self.load_next_u16(),
        }
    }
    pub fn read_address(&mut self, address: AddressType) -> u8 {
        let address = self.get_address_by(address);
        self.memory.read_u8(address)
    }

    pub fn write_address(&mut self, address: AddressType, val: u8) {
        let address = self.get_address_by(address);
        self.memory.write_u8(address, val);
    }

    pub fn read_address_u16(&mut self, address: AddressType) -> u16 {
        let address = self.get_address_by(address);
        self.memory.read_u16(address)
    }

    pub fn write_address_u16(&mut self, address: AddressType, val: u16) {
        let address = self.get_address_by(address);
        self.memory.write_u16(address, val);
    }
}

mod test {
    use super::*;

    #[test]
    fn memory_read_write_u16() {
        let mut memory = Memory::new();
        memory.write_u16(0x101, 0x100);
        assert_eq!(memory.read_u16(0x101), 0x100);
    }
    #[test]
    fn set_get_reg_test() {}

    #[test]
    fn reg_u16() {
        let mut registers = Register::new();
        registers.write_u8(RegisterU8Label::H, 1);
        registers.write_u8(RegisterU8Label::L, 1);
        println!("{} {}", registers.h, registers.l);
        assert_eq!(registers.read_u16(RegisterU16Label::HL), 0x101);
        registers.write_u16(RegisterU16Label::DE, 0x202);
        assert_eq!(registers.read_u16(RegisterU16Label::DE), 0x202);
    }
}
