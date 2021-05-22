use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// LdxImm: LDX immediate
pub struct LdxImm { }
impl Instruction for LdxImm {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xA2 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDX (Ldx) not implemented!");
        // Ok(())
    }
}

/// LdxZp: LDX zeropage
pub struct LdxZp { }
impl Instruction for LdxZp {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xA6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDX (Ldx) not implemented!");
        // Ok(())
    }
}

/// LdxAbs: LDX absolute
pub struct LdxAbs { }
impl Instruction for LdxAbs {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xAE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDX (Ldx) not implemented!");
        // Ok(())
    }
}

/// LdxZpY: LDX absolute, indexed by X
pub struct LdxZpY { }
impl Instruction for LdxZpY {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xB6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDX (Ldx) not implemented!");
        // Ok(())
    }
}

/// LdxAbsY: LDX absolute, indexed by Y
pub struct LdxAbsY { }
impl Instruction for LdxAbsY {
    fn opcode (&self) -> &'static str  { "LDX"}
    fn hexcode(&self) -> Byte { 0xBE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDX (Ldx) not implemented!");
        // Ok(())
    }
}
