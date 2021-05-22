use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Tya: TYA 
pub struct Tya { }
impl Instruction for Tya {
    fn opcode (&self) -> &'static str  { "TYA"}
    fn hexcode(&self) -> Byte { 0x98 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TYA (Tya) not implemented!");
        // Ok(())
    }
}
