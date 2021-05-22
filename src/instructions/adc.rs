use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };
use crate::instructions::generic::*;


/// AdcIndX: ADC indirect, indexed by X
pub struct AdcIndX { }
impl Instruction for AdcIndX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x61 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_indx_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcZp: ADC zeropage
pub struct AdcZp { }
impl Instruction for AdcZp {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x65 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_zp_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcImm: ADC immediate
pub struct AdcImm { }
impl Instruction for AdcImm {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x69 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_imm_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcAbs: ADC absolute
pub struct AdcAbs { }
impl Instruction for AdcAbs {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x6D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_abs_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcIndY: ADC indirect, indexed by Y
pub struct AdcIndY { }
impl Instruction for AdcIndY {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x71 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_indy_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcZpX: ADC absolute, indexed by X
pub struct AdcZpX { }
impl Instruction for AdcZpX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x75 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_zpx_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcAbsY: ADC absolute, indexed by Y
pub struct AdcAbsY { }
impl Instruction for AdcAbsY {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x79 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_absy_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}

/// AdcAbsX: ADC absolute, indexed by X
pub struct AdcAbsX { }
impl Instruction for AdcAbsX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x7D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = fetch_absx_val(cpu)?;
        add_to_acc(cpu, num);
        Ok(())
    }
}
