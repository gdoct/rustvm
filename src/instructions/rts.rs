use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Rts: RTS 
pub struct Rts { }
impl Instruction for Rts {
    fn opcode (&self) -> &'static str  { "RTS"}
    fn hexcode(&self) -> Byte { 0x60 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode RTS (Rts) not implemented!");
        // Ok(())
    }
}
