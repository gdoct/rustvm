use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Tsx: TSX 
pub struct Tsx { }
impl Instruction for Tsx {
    fn opcode (&self) -> &'static str  { "TSX"}
    fn hexcode(&self) -> Byte { 0xBA }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TSX (Tsx) not implemented!");
        // Ok(())
    }
}
