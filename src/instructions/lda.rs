use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, Word, CpuFlags };

/// LDA  Immediate     LDA #$44      $A9  2   2
pub struct LdaImm {}
impl Instruction for LdaImm {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA9 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = cpu.fetch_byte()?;
        cpu.set_a(val);
        Ok(())
    }
}

/// LDA Zero Page     LDA $44       $A5  2   3
pub struct LdaZp {}
impl Instruction for LdaZp {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA5 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_byte()? as Word;
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

/// LDA Zero Page,X   LDA $44,X     $B5  2   4
pub struct LdaZpX {}
impl Instruction for LdaZpX {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xB5 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let mut addr = (cpu.fetch_byte()? + cpu.get_x()) as Word;
        if addr > 0xff {
            addr -= 0xff;
            cpu.set_flag(CpuFlags::OVERFLOW, true)
        }
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

/// LDA Absolute      LDA $4400     $AD  3   4
pub struct LdaAbs {}
impl Instruction for LdaAbs {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xAD }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        let val = cpu.read_byte(addr);
        cpu.set_a(val);
        Ok(())
    }
}

/// LDA Absolute,X    LDA $4400,X   $BD  3   4+
pub struct LdaAbsX {}
impl Instruction for LdaAbsX {
    fn opcode (&self) -> &'static str  { "LDA"}
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

/// LDA Absolute,Y    LDA $4400,Y   $B9  3   4+
pub struct LdaAbsY {}
impl Instruction for LdaAbsY {
    fn opcode (&self) -> &'static str  { "LDA"}
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

/// LDA Indirect,X    LDA ($44,X)   $A1  2   6
pub struct LdaIndX {}
impl Instruction for LdaIndX {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA1 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_in_zp = cpu.fetch_byte()? as Word;
        let mut addr_plus_x = addr_in_zp + cpu.get_x() as Word;
        if addr_plus_x > 0xff {
            addr_plus_x -= 0xff;
        }
        let value_for_accum = cpu.read_byte(addr_plus_x);
        cpu.set_a(value_for_accum);
        Ok(())        
    }
}
/// LDA Indirect,Y    LDA ($44),Y   $B1  2   5+
pub struct LdaIndY {}
impl Instruction for LdaIndY {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xB1 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_in_zp = cpu.fetch_byte()? as Word;
        let mut addr_plus_y = addr_in_zp + cpu.get_x() as Word;
        if addr_plus_y > 0xff {
            addr_plus_y -= 0xff;
        }
        let value_for_accum = cpu.read_byte(addr_plus_y);
        cpu.set_a(value_for_accum);
        Ok(())
    }
}