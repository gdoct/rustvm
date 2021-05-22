use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Nop: NOP 
pub struct Nop { }
impl Instruction for Nop {
    fn opcode (&self) -> &'static str  { "NOP"}
    fn hexcode(&self) -> Byte { 0xEA }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        Ok(())
    }
}
