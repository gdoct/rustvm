use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// AndIndX: AND indirect, indexed by X
pub struct AndIndX { }
impl Instruction for AndIndX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x21 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndZp: AND zeropage
pub struct AndZp { }
impl Instruction for AndZp {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x25 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndImm: AND immediate
pub struct AndImm { }
impl Instruction for AndImm {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x29 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndAbs: AND absolute
pub struct AndAbs { }
impl Instruction for AndAbs {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x2D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndIndY: AND indirect, indexed by Y
pub struct AndIndY { }
impl Instruction for AndIndY {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x31 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndZpX: AND absolute, indexed by X
pub struct AndZpX { }
impl Instruction for AndZpX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x35 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndAbsY: AND absolute, indexed by Y
pub struct AndAbsY { }
impl Instruction for AndAbsY {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x39 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}

/// AndAbsX: AND absolute, indexed by X
pub struct AndAbsX { }
impl Instruction for AndAbsX {
    fn opcode (&self) -> &'static str  { "AND"}
    fn hexcode(&self) -> Byte { 0x3D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode AND (And) not implemented!");
        // Ok(())
    }
}
