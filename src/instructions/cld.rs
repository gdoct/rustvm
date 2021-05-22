use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Cld: CLD 
pub struct Cld { }
impl Instruction for Cld {
    fn opcode (&self) -> &'static str  { "CLD"}
    fn hexcode(&self) -> Byte { 0xD8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CLD (Cld) not implemented!");
        // Ok(())
    }
}
