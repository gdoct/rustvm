use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Pha: PHA 
pub struct Pha { }
impl Instruction for Pha {
    fn opcode (&self) -> &'static str  { "PHA"}
    fn hexcode(&self) -> Byte { 0x48 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode PHA (Pha) not implemented!");
        // Ok(())
    }
}
