use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Iny: INY 
pub struct Iny { }
impl Instruction for Iny {
    fn opcode (&self) -> &'static str  { "INY"}
    fn hexcode(&self) -> Byte { 0xC8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INY (Iny) not implemented!");
        // Ok(())
    }
}
