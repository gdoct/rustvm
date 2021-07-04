use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;

fn asl(cpu: &mut dyn VirtualCpu, num: Byte, with: Byte) {
    let asl = num << with;
    cpu.set_a(asl);
}

/// AslZp: ASL zeropage
/// Arithmetic Shift Left (ASL) operation between 
/// the content of Accumulator and the content of 
/// the zero page address
pub struct AslZp { }
impl Instruction for AslZp {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x06 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let zp = fetch_zp_val(cpu)?;
        let a = cpu.get_a();
        asl(cpu, a, zp);
        Ok(())
    }
}

/// Asl: ASL 
/// Arithmetic Shift Left (ASL)  operation between immediate val
/// and the content of the Accumulator
pub struct Asl { }
impl Instruction for Asl {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x0A }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let imm = fetch_imm_val(cpu)?;
        let a = cpu.get_a();
        asl(cpu, imm, a);
        Ok(())
    }
}

/// AslAbs: ASL absolute
/// Arithmetic Shift Left (ASL)  operation between the content of 
/// Accumulator and the content located at address $1234
pub struct AslAbs { }
impl Instruction for AslAbs {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x0E }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_abs_val(cpu)?;
        let a = cpu.get_a();
        asl(cpu, a, val);
        Ok(())
    }
}

/// AslZpX: ASL absolute, indexed by X
pub struct AslZpX { }
impl Instruction for AslZpX {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x16 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_absx_val(cpu)?;
        let a = cpu.get_a();
        asl(cpu, a, val);
        Ok(())
    }
}

/// AslAbsX: ASL absolute, indexed by X
/// Arithmetic Shift Left (ASL)  operation between the content of Accumulator and the content 
/// located at address calculated from $1234 adding content of X
pub struct AslAbsX { }
impl Instruction for AslAbsX {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x1E }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let val = fetch_absx_val(cpu)?;
        let a = cpu.get_a();
        asl(cpu, a, val);
        Ok(())
    }
}
