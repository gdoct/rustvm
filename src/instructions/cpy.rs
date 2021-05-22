use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// CpyImm: CPY immediate
pub struct CpyImm { }
impl Instruction for CpyImm {
    fn opcode (&self) -> &'static str  { "CPY"}
    fn hexcode(&self) -> Byte { 0xC0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPY (Cpy) not implemented!");
        // Ok(())
    }
}

/// CpyZp: CPY zeropage
pub struct CpyZp { }
impl Instruction for CpyZp {
    fn opcode (&self) -> &'static str  { "CPY"}
    fn hexcode(&self) -> Byte { 0xC4 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPY (Cpy) not implemented!");
        // Ok(())
    }
}

/// CpyAbs: CPY absolute
pub struct CpyAbs { }
impl Instruction for CpyAbs {
    fn opcode (&self) -> &'static str  { "CPY"}
    fn hexcode(&self) -> Byte { 0xCC }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode CPY (Cpy) not implemented!");
        // Ok(())
    }
}
