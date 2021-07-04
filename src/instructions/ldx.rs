use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;

fn ldx(cpu: &mut dyn VirtualCpu, val: Byte) {
    cpu.set_x(val);
}

/// LdxImm: LDX immediate
pub struct LdxImm { }
impl Instruction for LdxImm {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xA2 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_imm_val(cpu)?;
        ldx(cpu, val);
        Ok(())
    }
}

/// LdxZp: LDX zeropage
pub struct LdxZp { }
impl Instruction for LdxZp {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xA6 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_zp_val(cpu)?;
        ldx(cpu, val);
        Ok(())
    }
}

/// LdxAbs: LDX absolute
pub struct LdxAbs { }
impl Instruction for LdxAbs {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xAE }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_abs_val(cpu)?;
        ldx(cpu, val);
        Ok(())
    }
}

/// LdxZpY: LDX absolute, indexed by X
pub struct LdxZpY { }
impl Instruction for LdxZpY {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xB6 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_zpy_val(cpu)?;
        ldx(cpu, val);
        Ok(())
    }
}

/// LdxAbsY: LDX absolute, indexed by Y
pub struct LdxAbsY { }
impl Instruction for LdxAbsY {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xBE }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_absx_val(cpu)?;
        ldx(cpu, val);
        Ok(())
    }
}
