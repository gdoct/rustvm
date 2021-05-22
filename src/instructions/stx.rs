use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// StxZp: STX zeropage
pub struct StxZp { }
impl Instruction for StxZp {
    fn opcode (&self) -> &'static str  { "STX"}
    fn hexcode(&self) -> Byte { 0x86 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STX (Stx) not implemented!");
        // Ok(())
    }
}

/// StxAbs: STX absolute
pub struct StxAbs { }
impl Instruction for StxAbs {
    fn opcode (&self) -> &'static str  { "STX"}
    fn hexcode(&self) -> Byte { 0x8E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STX (Stx) not implemented!");
        // Ok(())
    }
}

/// StxZpY: STX absolute, indexed by X
pub struct StxZpY { }
impl Instruction for StxZpY {
    fn opcode (&self) -> &'static str  { "STX"}
    fn hexcode(&self) -> Byte { 0x96 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STX (Stx) not implemented!");
        // Ok(())
    }
}
