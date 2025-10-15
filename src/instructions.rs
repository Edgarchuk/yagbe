use crate::cpu::{Argument, Cpu, RegisterU16Label};

fn ld(cpu: &mut Cpu, args: [Argument; 2]) {
    match args[1] {
        Argument::RegisterU8(_) | Argument::Address(_) | Argument::ValueU8 => ld_u8(cpu, args),
        Argument::RegisterU16(_) | Argument::ValueU16 | Argument::AdjustedStackPointer => {
            ld_u16(cpu, args)
        }
    };
}

fn ld_u8(cpu: &mut Cpu, args: [Argument; 2]) {
    let source: u8 = match args[1] {
        Argument::RegisterU8(reg) => cpu.register.read_u8(reg),
        Argument::ValueU8 => cpu.load_next_u8(),
        Argument::Address(address) => cpu.read_address(address),
        _ => unreachable!("Error source type in ld_u8 instruction"),
    };
    match args[0] {
        Argument::RegisterU8(reg) => cpu.register.write_u8((reg), source),
        Argument::Address(address) => cpu.write_address((address), source),
        _ => unreachable!("Error distination type in ld_u8 istruction"),
    }
}

fn ld_u16(cpu: &mut Cpu, args: [Argument; 2]) {
    let source: u16 = match args[1] {
        Argument::RegisterU16(reg) => cpu.register.read_u16(reg),
        Argument::ValueU16 => cpu.load_next_u16(),
        Argument::AdjustedStackPointer => cpu.register.sp + cpu.load_next_u8() as u16,
        _ => unreachable!("Error source type in ld_u16 instruction"),
    };

    match args[0] {
        Argument::RegisterU16(reg) => cpu.register.write_u16(reg, source),
        Argument::Address(address) => cpu.write_address_u16(address, source),
        _ => unreachable!("Error distination type in ld_u16 istruction"),
    }
}

mod test_ld_instrution {
    use crate::cpu::{AddressType, Cpu, Memory, RegisterU8Label, RegisterU16Label};

    use super::*;

    #[test]
    fn ld_inst() {
        let mut cpu = Cpu::new(Memory::new());
        cpu.memory.write_u8(0x100, 11);
        cpu.register.write_u8(RegisterU8Label::L, 1);
        cpu.memory.write_u16(0x101, 0x100);
        cpu.register.pc = 0x101;
        ld(
            &mut cpu,
            [
                Argument::Address(AddressType::ValueU16),
                Argument::RegisterU8(RegisterU8Label::L),
            ],
        );
        assert_eq!(cpu.memory.read_u8(0x100), 1);

        cpu.register.pc = 0x101;
        cpu.memory.write_u16(0x101, 0x102);
        ld(
            &mut cpu,
            [
                Argument::RegisterU16(RegisterU16Label::DE),
                Argument::ValueU16,
            ],
        );
        assert_eq!(cpu.register.read_u16(RegisterU16Label::DE), 0x102)
    }
}

fn push(cpu: &mut Cpu, args: [Argument; 1]) {
    let mut register: RegisterU16Label;
    match args[0] {
        Argument::RegisterU16(reg) => register = reg,
        _ => unreachable!("Error source type in push instruction"),
    }
    let source = cpu.register.read_u16(register);

    cpu.register.sp -= 1;
    cpu.memory.write_u8(cpu.register.sp, (source >> 8) as u8);
    cpu.register.sp -= 1;
    cpu.memory.write_u8(cpu.register.sp, (source & 0xFF) as u8);
}

fn pop(cpu: &mut Cpu, args: [Argument; 1]) {
    let mut result: u16 = 0;
    result += (cpu.memory.read_u8(cpu.register.sp) as u16);
    cpu.register.sp += 1;
    result += (cpu.memory.read_u8(cpu.register.sp) as u16) << 8;
    cpu.register.sp += 1;

    match args[0] {
        Argument::RegisterU16(reg) => cpu.register.write_u16(reg, result),
        _ => unreachable!("Error source type in pop instrution"),
    }
}

mod test_push_instrution {
    use crate::cpu::Memory;

    use super::*;

    #[test]
    fn pop_push_inst() {
        let mut cpu = Cpu::new(Memory::new());

        cpu.register.write_u16(RegisterU16Label::HL, 0x102);
        push(&mut cpu, [Argument::RegisterU16(RegisterU16Label::HL)]);
        assert_eq!(cpu.memory.read_u16(cpu.register.sp), 0x102);
        pop(&mut cpu, [Argument::RegisterU16(RegisterU16Label::BC)]);
        assert_eq!(cpu.register.read_u16(RegisterU16Label::BC), 0x102)
    }
}
