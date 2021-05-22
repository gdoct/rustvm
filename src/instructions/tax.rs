use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Tax: TAX 
pub struct Tax { }
impl Instruction for Tax {
    fn opcode (&self) -> &'static str  { "TAX"}
    fn hexcode(&self) -> Byte { 0xAA }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TAX (Tax) not implemented!");
        // Ok(())
    }
}
