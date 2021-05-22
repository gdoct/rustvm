use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// IncZp: INC zeropage
pub struct IncZp { }
impl Instruction for IncZp {
    fn opcode (&self) -> &'static str  { "INC"}
    fn hexcode(&self) -> Byte { 0xE6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INC (Inc) not implemented!");
        // Ok(())
    }
}

/// IncAbs: INC absolute
pub struct IncAbs { }
impl Instruction for IncAbs {
    fn opcode (&self) -> &'static str  { "INC"}
    fn hexcode(&self) -> Byte { 0xEE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INC (Inc) not implemented!");
        // Ok(())
    }
}

/// IncZpX: INC absolute, indexed by X
pub struct IncZpX { }
impl Instruction for IncZpX {
    fn opcode (&self) -> &'static str  { "INC"}
    fn hexcode(&self) -> Byte { 0xF6 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INC (Inc) not implemented!");
        // Ok(())
    }
}

/// IncAbsX: INC absolute, indexed by X
pub struct IncAbsX { }
impl Instruction for IncAbsX {
    fn opcode (&self) -> &'static str  { "INC"}
    fn hexcode(&self) -> Byte { 0xFE }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode INC (Inc) not implemented!");
        // Ok(())
    }
}
