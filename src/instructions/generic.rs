use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, Word, CpuFlags };

pub fn fetch_imm_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    cpu.fetch_byte()
}

pub fn fetch_zp_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr = cpu.fetch_byte()? as Word;
    Ok(cpu.read_byte(addr))
}

pub fn fetch_zpx_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let mut addr = (cpu.fetch_byte()? + cpu.get_x()) as Word;
    if addr > 0xff {
        addr -= 0xff;
        cpu.set_flag(CpuFlags::OVERFLOW, true)
    }
    Ok(cpu.read_byte(addr))
}

pub fn fetch_zpy_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let mut addr = (cpu.fetch_byte()? + cpu.get_y()) as Word;
    if addr > 0xff {
        addr -= 0xff;
        cpu.set_flag(CpuFlags::OVERFLOW, true)
    }
    Ok(cpu.read_byte(addr))
}

pub fn fetch_abs_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr = cpu.fetch_word()?;
    Ok(cpu.read_byte(addr))
}

pub fn fetch_absx_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr = cpu.fetch_word()?;
    let add = cpu.get_x();
    let final_addr: Word = addr + (add as Word);
    Ok(cpu.read_byte(final_addr))
}

pub fn fetch_absy_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr = cpu.fetch_word()?;
    let add = cpu.get_y();
    let final_addr: Word = addr + (add as Word);
    Ok(cpu.read_byte(final_addr))
}

pub fn fetch_indx_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr_in_zp = cpu.fetch_byte()? as Word;
    let mut addr_plus_x = addr_in_zp + cpu.get_x() as Word;
    if addr_plus_x > 0xff {
        addr_plus_x -= 0xff;
    }
    Ok(cpu.read_byte(addr_plus_x))
}

pub fn fetch_indy_val(cpu: &mut dyn VirtualCpu) -> std::io::Result<Byte> {
    let addr_in_zp = cpu.fetch_byte()? as Word;
    let mut addr_plus_y = addr_in_zp + cpu.get_y() as Word;
    if addr_plus_y > 0xff {
        addr_plus_y -= 0xff;
    }
    Ok(cpu.read_byte(addr_plus_y))
}

pub fn add_to_acc(cpu: &mut dyn VirtualCpu, num: u8) {
    let a = cpu.get_a();
    let result = num + a;
    if result < num || result < a {
        cpu.set_flag(CpuFlags::OVERFLOW, true);
    }
    cpu.set_a(result);
}
