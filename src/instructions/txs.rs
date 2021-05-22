use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// Txs: TXS 
pub struct Txs { }
impl Instruction for Txs {
    fn opcode (&self) -> &'static str  { "TXS"}
    fn hexcode(&self) -> Byte { 0x9A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode TXS (Txs) not implemented!");
        // Ok(())
    }
}
