use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// CpxImm: CPX immediate
pub struct CpxImm { }
impl Instruction for CpxImm {
    fn opcode (&self) -> &'static str  { "CPX"}
    fn hexcode(&self) -> Byte { 0xE0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPX (Cpx) not implemented!");
        // Ok(())
    }
}

/// CpxZp: CPX zeropage
pub struct CpxZp { }
impl Instruction for CpxZp {
    fn opcode (&self) -> &'static str  { "CPX"}
    fn hexcode(&self) -> Byte { 0xE4 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPX (Cpx) not implemented!");
        // Ok(())
    }
}

/// CpxAbs: CPX absolute
pub struct CpxAbs { }
impl Instruction for CpxAbs {
    fn opcode (&self) -> &'static str  { "CPX"}
    fn hexcode(&self) -> Byte { 0xEC }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPX (Cpx) not implemented!");
        // Ok(())
    }
}
