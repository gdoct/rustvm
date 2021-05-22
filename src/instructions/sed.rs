use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Sed: SED 
pub struct Sed { }
impl Instruction for Sed {
    fn opcode (&self) -> &'static str  { "SED"}
    fn hexcode(&self) -> Byte { 0xF8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SED (Sed) not implemented!");
        // Ok(())
    }
}
