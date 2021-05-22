use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Clv: CLV 
pub struct Clv { }
impl Instruction for Clv {
    fn opcode (&self) -> &'static str  { "CLV"}
    fn hexcode(&self) -> Byte { 0xB8 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CLV (Clv) not implemented!");
        // Ok(())
    }
}
