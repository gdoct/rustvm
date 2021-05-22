use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Rti: RTI 
pub struct Rti { }
impl Instruction for Rti {
    fn opcode (&self) -> &'static str  { "RTI"}
    fn hexcode(&self) -> Byte { 0x40 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode RTI (Rti) not implemented!");
        // Ok(())
    }
}
