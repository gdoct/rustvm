use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BcsRel: BCS relative
pub struct BcsRel { }
impl Instruction for BcsRel {
    fn opcode (&self) -> &'static str  { "BCS"}
    fn hexcode(&self) -> Byte { 0xB0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BCS (Bcs) not implemented!");
        // Ok(())
    }
}
