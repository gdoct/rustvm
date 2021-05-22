use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Dex: DEX 
pub struct Dex { }
impl Instruction for Dex {
    fn opcode (&self) -> &'static str  { "DEX"}
    fn hexcode(&self) -> Byte { 0xCA }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEX (Dex) not implemented!");
        // Ok(())
    }
}
