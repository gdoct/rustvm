use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// CmpIndX: CMP indirect, indexed by X
pub struct CmpIndX { }
impl Instruction for CmpIndX {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xC1 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpZp: CMP zeropage
pub struct CmpZp { }
impl Instruction for CmpZp {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xC5 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpImm: CMP immediate
pub struct CmpImm { }
impl Instruction for CmpImm {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xC9 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpAbs: CMP absolute
pub struct CmpAbs { }
impl Instruction for CmpAbs {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xCD }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpIndY: CMP indirect, indexed by Y
pub struct CmpIndY { }
impl Instruction for CmpIndY {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xD1 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpZpX: CMP absolute, indexed by X
pub struct CmpZpX { }
impl Instruction for CmpZpX {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xD5 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpAbsY: CMP absolute, indexed by Y
pub struct CmpAbsY { }
impl Instruction for CmpAbsY {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xD9 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}

/// CmpAbsX: CMP absolute, indexed by X
pub struct CmpAbsX { }
impl Instruction for CmpAbsX {
    fn opcode (&self) -> &'static str  { "CMP"}
    fn hexcode(&self) -> Byte { 0xDD }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CMP (Cmp) not implemented!");
        // Ok(())
    }
}
