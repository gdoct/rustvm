use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Tay: TAY 
pub struct Tay { }
impl Instruction for Tay {
    fn opcode (&self) -> &'static str  { "TAY"}
    fn hexcode(&self) -> Byte { 0xA8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TAY (Tay) not implemented!");
        // Ok(())
    }
}
