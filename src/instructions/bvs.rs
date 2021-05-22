use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BvsRel: BVS relative
pub struct BvsRel { }
impl Instruction for BvsRel {
    fn opcode (&self) -> &'static str  { "BVS"}
    fn hexcode(&self) -> Byte { 0x70 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BVS (Bvs) not implemented!");
        // Ok(())
    }
}
