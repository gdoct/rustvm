use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Php: PHP 
pub struct Php { }
impl Instruction for Php {
    fn opcode (&self) -> &'static str  { "PHP"}
    fn hexcode(&self) -> Byte { 0x08 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode PHP (Php) not implemented!");
        // Ok(())
    }
}
