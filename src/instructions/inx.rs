use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Inx: INX 
pub struct Inx { }
impl Instruction for Inx {
    fn opcode (&self) -> &'static str  { "INX"}
    fn hexcode(&self) -> Byte { 0xE8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INX (Inx) not implemented!");
        // Ok(())
    }
}
