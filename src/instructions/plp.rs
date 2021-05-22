use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Plp: PLP 
pub struct Plp { }
impl Instruction for Plp {
    fn opcode (&self) -> &'static str  { "PLP"}
    fn hexcode(&self) -> Byte { 0x28 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode PLP (Plp) not implemented!");
        // Ok(())
    }
}
