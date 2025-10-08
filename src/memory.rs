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
            checksum = checksum.wrapping_sub(self.get(idx)).wrapping_sub(1);
        }
        println!("{} {}", checksum, self.get(0x014Du16));
        if checksum != self.get(0x014Du16) {
            return Err(MemoryError::Checksum8NotEq);
        }
        Ok(())
    }

    pub fn get(&self, address: u16) -> u8 {
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

    pub fn set(&mut self, address: u16, value: u8) {
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
}

struct Cpu {
    memory: Memory,
    register: Regiter,
}

#[derive(Default)]
struct Regiter {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Regiter {
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
    fn get_multi_reg(&self, regist: MultiRegisterType) -> u16 {
        match regist {
            MultiRegisterType::AF => (self.a as u16) << 8 + self.f,
            MultiRegisterType::BC => (self.b as u16) << 8 + self.c,
            MultiRegisterType::DE => (self.d as u16) << 8 + self.e,
            MultiRegisterType::HL => (self.h as u16) << 8 + self.l,
            MultiRegisterType::SP => self.sp,
            MultiRegisterType::PC => self.pc,
        }
    }
    fn get_reg(&self, regist: RegisterType) -> u8 {
        match regist {
            RegisterType::A => self.a,
            RegisterType::F => self.f,
            RegisterType::B => self.b,
            RegisterType::C => self.c,
            RegisterType::D => self.d,
            RegisterType::E => self.e,
            RegisterType::H => self.h,
            RegisterType::L => self.l,
        }
    }
    fn set_reg(&mut self, regist: RegisterType, value: u8) {
        match regist {
            RegisterType::A => self.a = value,
            RegisterType::F => self.f = value,
            RegisterType::B => self.b = value,
            RegisterType::C => self.c = value,
            RegisterType::D => self.d = value,
            RegisterType::E => self.e = value,
            RegisterType::H => self.h = value,
            RegisterType::L => self.l = value,
        }
    }
}

enum RegisterType {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

enum MultiRegisterType {
    SP,
    PC,
    AF,
    BC,
    DE,
    HL,
}

enum AddressType {
    Direct(u16),
    MultiRegister(MultiRegisterType),
}

enum ValueType {
    Direct(u8),
    Register(RegisterType),
    Address(AddressType),
}

impl Cpu {
    fn new(memory: Memory) -> Self {
        Self {
            memory: memory,
            register: Regiter::new(),
        }
    }

    fn get(&self, value: ValueType) -> u8 {
        match value {
            ValueType::Direct(v) => v,
            ValueType::Register(r) => self.register.get_reg(r),
            ValueType::Address(AddressType::Direct(add)) => self.memory.get(add),
            ValueType::Address(AddressType::MultiRegister(mr)) => {
                self.memory.get(self.register.get_multi_reg(mr))
            }
        }
    }

    fn set(&mut self, value: ValueType, ) {
        match value
    }
}
