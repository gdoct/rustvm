use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// LsrZp: LSR zeropage
pub struct LsrZp { }
impl Instruction for LsrZp {
    fn opcode (&self) -> &'static str  { "LSR"}
    fn hexcode(&self) -> Byte { 0x46 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LSR (Lsr) not implemented!");
        // Ok(())
    }
}

/// Lsr: LSR 
pub struct Lsr { }
impl Instruction for Lsr {
    fn opcode (&self) -> &'static str  { "LSR"}
    fn hexcode(&self) -> Byte { 0x4A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LSR (Lsr) not implemented!");
        // Ok(())
    }
}

/// LsrAbs: LSR absolute
pub struct LsrAbs { }
impl Instruction for LsrAbs {
    fn opcode (&self) -> &'static str  { "LSR"}
    fn hexcode(&self) -> Byte { 0x4E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LSR (Lsr) not implemented!");
        // Ok(())
    }
}

/// LsrZpX: LSR absolute, indexed by X
pub struct LsrZpX { }
impl Instruction for LsrZpX {
    fn opcode (&self) -> &'static str  { "LSR"}
    fn hexcode(&self) -> Byte { 0x56 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LSR (Lsr) not implemented!");
        // Ok(())
    }
}

/// LsrAbsX: LSR absolute, indexed by X
pub struct LsrAbsX { }
impl Instruction for LsrAbsX {
    fn opcode (&self) -> &'static str  { "LSR"}
    fn hexcode(&self) -> Byte { 0x5E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LSR (Lsr) not implemented!");
        // Ok(())
    }
}
