use crate::types::{ Byte, Word,  CpuFlags };
use crate::traits::{ VirtualCpu, Instruction };
use std::io::{ Error, ErrorKind };

pub fn parse_opcode(opcode: Byte) -> std::result::Result<Box<dyn Instruction + 'static>, Error> {
    match opcode {
        0x00 => { Ok(Box::new(BrkInstruction {})) }
        0xea => { Ok(Box::new(NopInstruction {})) }
        0x4c => { Ok(Box::new(JmpAbsInstruction {})) }
        0x6c => { Ok(Box::new(JmpIndInstruction {})) }

        // LDA
        0xa9 => { Ok(Box::new(LdaImmInstruction {})) }
        0xa5 => { Ok(Box::new(LdaZpInstruction {})) }
        0xb5 => { Ok(Box::new(LdaZpXInstruction {})) }
        0xad => { Ok(Box::new(LdaAbsInstruction {})) }
        0xbd => { Ok(Box::new(LdaAbsXInstruction {})) }
        0xb9 => { Ok(Box::new(LdaAbsYInstruction {})) }

        _ => { Err(Error::new(ErrorKind::Other, format!("opcode {} not implemented", opcode))) }
    }
}
/**
Immediate     LDA #$44      $A9  2   2
Zero Page     LDA $44       $A5  2   3
Zero Page,X   LDA $44,X     $B5  2   4
Absolute      LDA $4400     $AD  3   4
Absolute,X    LDA $4400,X   $BD  3   4+
Absolute,Y    LDA $4400,Y   $B9  3   4+
 */
pub struct NopInstruction {}
impl Instruction for NopInstruction {
    fn opcode (&self) -> &str  { "NOP"}
    fn hexcode (&self) -> Byte { 0xea }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        // nop
        Ok(())
    }
}

pub struct BrkInstruction {}
impl Instruction for BrkInstruction {
    fn opcode (&self) -> &str  { "BRK"}
    fn hexcode (&self) -> Byte { 0x00 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        cpu.set_flag(CpuFlags::BREAK, true);
        Ok(())
    }
}

pub struct JmpAbsInstruction {}
impl Instruction for JmpAbsInstruction {
    fn opcode (&self) -> &str  { "JMP"}
    fn hexcode (&self) -> Byte { 0x4c }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        cpu.set_pc(addr);
        Ok(())
    }
}

pub struct JmpIndInstruction {}
impl Instruction for JmpIndInstruction {
    fn opcode (&self) -> &str { "JMP"}
    fn hexcode (&self) -> Byte { 0x6c }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_of_addr = cpu.fetch_word()?;
        let addr = cpu.read_word(addr_of_addr);
        cpu.set_pc(addr);
        Ok(())
    }
}

/** LDA
Immediate     LDA #$44      $A9  2   2
Zero Page     LDA $44       $A5  2   3
Zero Page,X   LDA $44,X     $B5  2   4
Absolute      LDA $4400     $AD  3   4
Absolute,X    LDA $4400,X   $BD  3   4+
Absolute,Y    LDA $4400,Y   $B9  3   4+
Indirect,X    LDA ($44,X)   $A1  2   6
Indirect,Y    LDA ($44),Y   $B1  2   5+ */

pub struct LdaImmInstruction {}
impl Instruction for LdaImmInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA9 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = cpu.fetch_byte()?;
        cpu.set_a(val);
        Ok(())
    }
}

pub struct LdaZpInstruction {}
impl Instruction for LdaZpInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA5 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_byte()? as Word;
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

pub struct LdaZpXInstruction {}
impl Instruction for LdaZpXInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xB5 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = (cpu.fetch_byte()? + cpu.get_x()) as Word;
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

pub struct LdaAbsInstruction {}
impl Instruction for LdaAbsInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xAD }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

pub struct LdaAbsXInstruction {}
impl Instruction for LdaAbsXInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xBD }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        let add = cpu.get_x();
        let final_addr: Word = addr + (add as Word);
        let val = cpu.read_byte(final_addr);
        cpu.set_a(val);
        Ok(())
    }
}

pub struct LdaAbsYInstruction {}
impl Instruction for LdaAbsYInstruction {
    fn opcode (&self) -> &str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xB9 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        let add = cpu.get_y();
        let final_addr: Word = addr + (add as Word);
        let val = cpu.read_byte(final_addr);
        cpu.set_a(val);
        Ok(())
    }
}