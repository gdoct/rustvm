use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Dey: DEY 
pub struct Dey { }
impl Instruction for Dey {
    fn opcode (&self) -> &'static str  { "DEY"}
    fn hexcode(&self) -> Byte { 0x88 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEY (Dey) not implemented!");
        // Ok(())
    }
}
