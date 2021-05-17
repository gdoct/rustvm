use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// JMP: jump to an absolute address
pub struct JmpAbs {}
impl Instruction for JmpAbs {
    fn opcode (&self) -> &'static str  { "JMP"}
    fn hexcode (&self) -> Byte { 0x4c }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        cpu.set_pc(addr);
        Ok(())
    }
}

/// JMP: jump to an address stored in memory
pub struct JmpInd {}
impl Instruction for JmpInd {
    fn opcode (&self) -> &'static str { "JMP"}
    fn hexcode (&self) -> Byte { 0x6c }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_of_addr = cpu.fetch_word()?;
        let addr = cpu.read_word(addr_of_addr);
        cpu.set_pc(addr);
        Ok(())
    }
}