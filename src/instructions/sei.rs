use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Sei: SEI 
pub struct Sei { }
impl Instruction for Sei {
    fn opcode (&self) -> &'static str  { "SEI"}
    fn hexcode(&self) -> Byte { 0x78 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SEI (Sei) not implemented!");
        // Ok(())
    }
}
