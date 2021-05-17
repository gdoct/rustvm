use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, CpuFlags };

/// BRK: set the cpu's BREAK flag
pub struct Brk {}
impl Instruction for Brk {
    fn opcode (&self) -> &'static str  { "BRK"}
    fn hexcode (&self) -> Byte { 0x00 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        cpu.set_flag(CpuFlags::BREAK, true);
        Ok(())
    }
}

/// NOP: go to the next clock cycle
pub struct Nop {}
impl Instruction for Nop {
    fn opcode (&self) -> &'static str  { "NOP"}
    fn hexcode (&self) -> Byte { 0xea }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        // nop
        Ok(())
    }
}