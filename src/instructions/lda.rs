use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;

/// LDA  Immediate     LDA #$44      $A9  2   2
pub struct LdaImm {}
impl Instruction for LdaImm {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xA9 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_imm_val(cpu)?;
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
        let val = fetch_zp_val(cpu)?;
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
        let val = fetch_zpx_val(cpu)?;
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
        let val = fetch_abs_val(cpu)?;
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
        let val = fetch_absx_val(cpu)?;
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
        let val = fetch_absy_val(cpu)?;
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
        let val = fetch_indx_val(cpu)?;
        cpu.set_a(val);
        Ok(())        
    }
}
/// LDA Indirect,Y    LDA ($44),Y   $B1  2   5+
pub struct LdaIndY {}
impl Instruction for LdaIndY {
    fn opcode (&self) -> &'static str  { "LDA"}
    fn hexcode (&self) -> Byte { 0xB1 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_indy_val(cpu)?;
        cpu.set_a(val);
        Ok(())
    }
}