use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BccRel: BCC relative
pub struct BccRel { }
impl Instruction for BccRel {
    fn opcode (&self) -> &'static str  { "BCC"}
    fn hexcode(&self) -> Byte { 0x90 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BCC (Bcc) not implemented!");
        // Ok(())
    }
}
