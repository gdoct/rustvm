use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Cli: CLI 
pub struct Cli { }
impl Instruction for Cli {
    fn opcode (&self) -> &'static str  { "CLI"}
    fn hexcode(&self) -> Byte { 0x58 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CLI (Cli) not implemented!");
        // Ok(())
    }
}
