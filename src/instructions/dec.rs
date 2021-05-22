use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// DecZp: DEC zeropage
pub struct DecZp { }
impl Instruction for DecZp {
    fn opcode (&self) -> &'static str  { "DEC"}
    fn hexcode(&self) -> Byte { 0xC6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEC (Dec) not implemented!");
        // Ok(())
    }
}

/// DecAbs: DEC absolute
pub struct DecAbs { }
impl Instruction for DecAbs {
    fn opcode (&self) -> &'static str  { "DEC"}
    fn hexcode(&self) -> Byte { 0xCE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEC (Dec) not implemented!");
        // Ok(())
    }
}

/// DecZpX: DEC absolute, indexed by X
pub struct DecZpX { }
impl Instruction for DecZpX {
    fn opcode (&self) -> &'static str  { "DEC"}
    fn hexcode(&self) -> Byte { 0xD6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEC (Dec) not implemented!");
        // Ok(())
    }
}

/// DecAbsX: DEC absolute, indexed by X
pub struct DecAbsX { }
impl Instruction for DecAbsX {
    fn opcode (&self) -> &'static str  { "DEC"}
    fn hexcode(&self) -> Byte { 0xDE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode DEC (Dec) not implemented!");
        // Ok(())
    }
}
