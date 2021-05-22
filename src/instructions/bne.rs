use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BneRel: BNE relative
pub struct BneRel { }
impl Instruction for BneRel {
    fn opcode (&self) -> &'static str  { "BNE"}
    fn hexcode(&self) -> Byte { 0xD0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BNE (Bne) not implemented!");
        // Ok(())
    }
}
