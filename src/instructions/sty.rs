use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// StyZp: STY zeropage
pub struct StyZp { }
impl Instruction for StyZp {
    fn opcode (&self) -> &'static str  { "STY"}
    fn hexcode(&self) -> Byte { 0x84 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STY (Sty) not implemented!");
        // Ok(())
    }
}

/// StyAbs: STY absolute
pub struct StyAbs { }
impl Instruction for StyAbs {
    fn opcode (&self) -> &'static str  { "STY"}
    fn hexcode(&self) -> Byte { 0x8C }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STY (Sty) not implemented!");
        // Ok(())
    }
}

/// StyZpX: STY absolute, indexed by X
pub struct StyZpX { }
impl Instruction for StyZpX {
    fn opcode (&self) -> &'static str  { "STY"}
    fn hexcode(&self) -> Byte { 0x94 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode STY (Sty) not implemented!");
        // Ok(())
    }
}
