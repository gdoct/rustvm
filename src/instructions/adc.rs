use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, Word, CpuFlags };

/// AdcIndX: ADC indirect, indexed by X
pub struct AdcIndX { }
impl Instruction for AdcIndX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x61 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}

/// AdcZp: ADC zeropage
pub struct AdcZp { }
impl Instruction for AdcZp {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x65 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_byte()? as Word;
        let num = cpu.read_byte(addr);
        let a = cpu.get_a();
        let result = num + a;
        if result < num || result < a {
            cpu.set_flag(CpuFlags::OVERFLOW, true);
        }
        cpu.set_a(result);
        Ok(())
    }
}

/// AdcImm: ADC immediate
pub struct AdcImm { }
impl Instruction for AdcImm {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x69 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let num = cpu.fetch_byte()?;
        let a = cpu.get_a();
        let result = num + a;
        if result < num || result < a {
            cpu.set_flag(CpuFlags::OVERFLOW, true);
        }
        cpu.set_a(result);
        Ok(())
    }
}

/// AdcAbs: ADC absolute
pub struct AdcAbs { }
impl Instruction for AdcAbs {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x6D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}

/// AdcIndY: ADC indirect, indexed by Y
pub struct AdcIndY { }
impl Instruction for AdcIndY {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x71 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}

/// AdcZpX: ADC absolute, indexed by X
pub struct AdcZpX { }
impl Instruction for AdcZpX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x75 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}

/// AdcAbsY: ADC absolute, indexed by Y
pub struct AdcAbsY { }
impl Instruction for AdcAbsY {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x79 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}

/// AdcAbsX: ADC absolute, indexed by X
pub struct AdcAbsX { }
impl Instruction for AdcAbsX {
    fn opcode (&self) -> &'static str  { "ADC"}
    fn hexcode(&self) -> Byte { 0x7D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ADC (Adc) not implemented!");
        // Ok(())
    }
}
