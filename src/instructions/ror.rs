use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// RorZp: ROR zeropage
pub struct RorZp { }
impl Instruction for RorZp {
    fn opcode (&self) -> &'static str  { "ROR"}
    fn hexcode(&self) -> Byte { 0x66 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROR (Ror) not implemented!");
        // Ok(())
    }
}

/// Ror: ROR 
pub struct Ror { }
impl Instruction for Ror {
    fn opcode (&self) -> &'static str  { "ROR"}
    fn hexcode(&self) -> Byte { 0x6A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROR (Ror) not implemented!");
        // Ok(())
    }
}

/// RorAbs: ROR absolute
pub struct RorAbs { }
impl Instruction for RorAbs {
    fn opcode (&self) -> &'static str  { "ROR"}
    fn hexcode(&self) -> Byte { 0x6E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROR (Ror) not implemented!");
        // Ok(())
    }
}

/// RorZpX: ROR absolute, indexed by X
pub struct RorZpX { }
impl Instruction for RorZpX {
    fn opcode (&self) -> &'static str  { "ROR"}
    fn hexcode(&self) -> Byte { 0x76 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROR (Ror) not implemented!");
        // Ok(())
    }
}

/// RorAbsX: ROR absolute, indexed by X
pub struct RorAbsX { }
impl Instruction for RorAbsX {
    fn opcode (&self) -> &'static str  { "ROR"}
    fn hexcode(&self) -> Byte { 0x7E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROR (Ror) not implemented!");
        // Ok(())
    }
}
