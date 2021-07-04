use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, CpuFlags };
use crate::instructions::generic::*;
/// BmiRel: BMI relative
pub struct BmiRel { }
impl Instruction for BmiRel {
    fn opcode (&self) -> &'static str  { "BMI"}
    fn hexcode(&self) -> Byte { 0x30 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let there = fetch_imm_val(cpu)?;
        let hascarry = cpu.get_flag(CpuFlags::CARRY);
        if hascarry {
            jump_relative(cpu, there);
        }
        Ok(())
    }
}
