use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Clc: CLC 
pub struct Clc { }
impl Instruction for Clc {
    fn opcode (&self) -> &'static str  { "CLC"}
    fn hexcode(&self) -> Byte { 0x18 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CLC (Clc) not implemented!");
        // Ok(())
    }
}
