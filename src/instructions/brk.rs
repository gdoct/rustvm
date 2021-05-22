use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, CpuFlags };

/// Brk: BRK 
pub struct Brk { }
impl Instruction for Brk {
    fn opcode (&self) -> &'static str  { "BRK"}
    fn hexcode(&self) -> Byte { 0x00 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        cpu.set_flag(CpuFlags::BREAK, true);
        Ok(())
    }
}
