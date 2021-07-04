use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;

fn ldy(cpu: &mut dyn VirtualCpu, val: Byte) {
    cpu.set_y(val);
}

/// LdyImm: LDY immediate
pub struct LdyImm { }
impl Instruction for LdyImm {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xA0 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_imm_val(cpu)?;
        ldy(cpu, val);
        Ok(())
    }
}

/// LdyZp: LDY zeropage
pub struct LdyZp { }
impl Instruction for LdyZp {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xA4 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_zp_val(cpu)?;
        ldy(cpu, val);
        Ok(())
    }
}

/// LdyAbs: LDY absolute
pub struct LdyAbs { }
impl Instruction for LdyAbs {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xAC }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_abs_val(cpu)?;
        ldy(cpu, val);
        Ok(())
    }
}

/// LdyZpX: LDY absolute, indexed by X
pub struct LdyZpX { }
impl Instruction for LdyZpX {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xB4 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_zpx_val(cpu)?;
        ldy(cpu, val);
        Ok(())
    }
}

/// LdyAbsX: LDY absolute, indexed by X
pub struct LdyAbsX { }
impl Instruction for LdyAbsX {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xBC }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_absx_val(cpu)?;
        ldy(cpu, val);
        Ok(())
    }
}
