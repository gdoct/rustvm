use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Sec: SEC 
pub struct Sec { }
impl Instruction for Sec {
    fn opcode (&self) -> &'static str  { "SEC"}
    fn hexcode(&self) -> Byte { 0x38 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SEC (Sec) not implemented!");
        // Ok(())
    }
}
