use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, CpuFlags };
use crate::instructions::generic::*;
/// BccRel: BCC relative
/// Branch on Carry Clear
pub struct BccRel { }
impl Instruction for BccRel {
    fn opcode (&self) -> &'static str  { "BCC"}
    fn hexcode(&self) -> Byte { 0x90 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let there = fetch_imm_val(cpu)?;
        let hascarry = cpu.get_flag(CpuFlags::CARRY);
        if !hascarry {
            jump_relative(cpu, there);
        }
        Ok(())
    }
}
