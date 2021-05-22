use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Txa: TXA 
pub struct Txa { }
impl Instruction for Txa {
    fn opcode (&self) -> &'static str  { "TXA"}
    fn hexcode(&self) -> Byte { 0x8A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TXA (Txa) not implemented!");
        // Ok(())
    }
}
