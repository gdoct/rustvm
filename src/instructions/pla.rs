use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Pla: PLA 
pub struct Pla { }
impl Instruction for Pla {
    fn opcode (&self) -> &'static str  { "PLA"}
    fn hexcode(&self) -> Byte { 0x68 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode PLA (Pla) not implemented!");
        // Ok(())
    }
}
