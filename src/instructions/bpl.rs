use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BplRel: BPL relative
pub struct BplRel { }
impl Instruction for BplRel {
    fn opcode (&self) -> &'static str  { "BPL"}
    fn hexcode(&self) -> Byte { 0x10 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BPL (Bpl) not implemented!");
        // Ok(())
    }
}
