use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;

fn and(cpu: &mut dyn VirtualCpu, num: Byte) {
    let val = cpu.get_a() & num;
    cpu.set_a(val)
}

/// AndIndX: AND indirect, indexed by X
pub struct AndIndX { }
impl Instruction for AndIndX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x21 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_indx_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndZp: AND zeropage
pub struct AndZp { }
impl Instruction for AndZp {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x25 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_zp_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndImm: AND immediate
pub struct AndImm { }
impl Instruction for AndImm {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x29 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_imm_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndAbs: AND absolute
pub struct AndAbs { }
impl Instruction for AndAbs {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x2D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_abs_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndIndY: AND indirect, indexed by Y
pub struct AndIndY { }
impl Instruction for AndIndY {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x31 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_indy_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndZpX: AND absolute, indexed by X
pub struct AndZpX { }
impl Instruction for AndZpX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x35 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_zpx_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndAbsY: AND absolute, indexed by Y
pub struct AndAbsY { }
impl Instruction for AndAbsY {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x39 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_absy_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}

/// AndAbsX: AND absolute, indexed by X
pub struct AndAbsX { }
impl Instruction for AndAbsX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x3D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_absx_val(cpu)?;
        and(cpu, num);
        Ok(())
    }
}
