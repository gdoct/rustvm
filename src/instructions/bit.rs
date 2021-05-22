use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BitZp: BIT zeropage
pub struct BitZp { }
impl Instruction for BitZp {
    fn opcode (&self) -> &'static str  { "BIT"}
    fn hexcode(&self) -> Byte { 0x24 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BIT (Bit) not implemented!");
        // Ok(())
    }
}

/// BitAbs: BIT absolute
pub struct BitAbs { }
impl Instruction for BitAbs {
    fn opcode (&self) -> &'static str  { "BIT"}
    fn hexcode(&self) -> Byte { 0x2C }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BIT (Bit) not implemented!");
        // Ok(())
    }
}
