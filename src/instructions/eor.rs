use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// EorIndX: EOR indirect, indexed by X
pub struct EorIndX { }
impl Instruction for EorIndX {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x41 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorZp: EOR zeropage
pub struct EorZp { }
impl Instruction for EorZp {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x45 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorImm: EOR immediate
pub struct EorImm { }
impl Instruction for EorImm {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x49 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorAbs: EOR absolute
pub struct EorAbs { }
impl Instruction for EorAbs {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x4D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorIndY: EOR indirect, indexed by Y
pub struct EorIndY { }
impl Instruction for EorIndY {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x51 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorZpX: EOR absolute, indexed by X
pub struct EorZpX { }
impl Instruction for EorZpX {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x55 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorAbsY: EOR absolute, indexed by Y
pub struct EorAbsY { }
impl Instruction for EorAbsY {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x59 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}

/// EorAbsX: EOR absolute, indexed by X
pub struct EorAbsX { }
impl Instruction for EorAbsX {
    fn opcode (&self) -> &'static str  { "EOR"}
    fn hexcode(&self) -> Byte { 0x5D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode EOR (Eor) not implemented!");
        // Ok(())
    }
}
