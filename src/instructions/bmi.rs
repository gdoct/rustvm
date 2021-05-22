use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// BmiRel: BMI relative
pub struct BmiRel { }
impl Instruction for BmiRel {
    fn opcode (&self) -> &'static str  { "BMI"}
    fn hexcode(&self) -> Byte { 0x30 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode BMI (Bmi) not implemented!");
        // Ok(())
    }
}
