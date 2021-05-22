use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BeqRel: BEQ relative
pub struct BeqRel { }
impl Instruction for BeqRel {
    fn opcode (&self) -> &'static str  { "BEQ"}
    fn hexcode(&self) -> Byte { 0xF0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BEQ (Beq) not implemented!");
        // Ok(())
    }
}
