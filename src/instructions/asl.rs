use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// AslZp: ASL zeropage
pub struct AslZp { }
impl Instruction for AslZp {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x06 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ASL (Asl) not implemented!");
        // Ok(())
    }
}

/// Asl: ASL 
pub struct Asl { }
impl Instruction for Asl {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x0A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ASL (Asl) not implemented!");
        // Ok(())
    }
}

/// AslAbs: ASL absolute
pub struct AslAbs { }
impl Instruction for AslAbs {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x0E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ASL (Asl) not implemented!");
        // Ok(())
    }
}

/// AslZpX: ASL absolute, indexed by X
pub struct AslZpX { }
impl Instruction for AslZpX {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x16 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ASL (Asl) not implemented!");
        // Ok(())
    }
}

/// AslAbsX: ASL absolute, indexed by X
pub struct AslAbsX { }
impl Instruction for AslAbsX {
    fn opcode (&self) -> &'static str  { "ASL"}
    fn hexcode(&self) -> Byte { 0x1E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ASL (Asl) not implemented!");
        // Ok(())
    }
}
