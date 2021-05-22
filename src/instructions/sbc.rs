use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// SbcIndX: SBC indirect, indexed by X
pub struct SbcIndX { }
impl Instruction for SbcIndX {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xE1 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcZp: SBC zeropage
pub struct SbcZp { }
impl Instruction for SbcZp {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xE5 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcImm: SBC immediate
pub struct SbcImm { }
impl Instruction for SbcImm {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xE9 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcAbs: SBC absolute
pub struct SbcAbs { }
impl Instruction for SbcAbs {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xED }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcIndY: SBC indirect, indexed by Y
pub struct SbcIndY { }
impl Instruction for SbcIndY {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xF1 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcZpX: SBC absolute, indexed by X
pub struct SbcZpX { }
impl Instruction for SbcZpX {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xF5 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcAbsY: SBC absolute, indexed by Y
pub struct SbcAbsY { }
impl Instruction for SbcAbsY {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xF9 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}

/// SbcAbsX: SBC absolute, indexed by X
pub struct SbcAbsX { }
impl Instruction for SbcAbsX {
    fn opcode (&self) -> &'static str  { "SBC"}
    fn hexcode(&self) -> Byte { 0xFD }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode SBC (Sbc) not implemented!");
        // Ok(())
    }
}
