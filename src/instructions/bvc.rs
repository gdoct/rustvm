use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BvcRel: BVC relative
pub struct BvcRel { }
impl Instruction for BvcRel {
    fn opcode (&self) -> &'static str  { "BVC"}
    fn hexcode(&self) -> Byte { 0x50 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BVC (Bvc) not implemented!");
        // Ok(())
    }
}
