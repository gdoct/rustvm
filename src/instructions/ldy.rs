use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// LdyImm: LDY immediate
pub struct LdyImm { }
impl Instruction for LdyImm {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xA0 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDY (Ldy) not implemented!");
        // Ok(())
    }
}

/// LdyZp: LDY zeropage
pub struct LdyZp { }
impl Instruction for LdyZp {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xA4 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDY (Ldy) not implemented!");
        // Ok(())
    }
}

/// LdyAbs: LDY absolute
pub struct LdyAbs { }
impl Instruction for LdyAbs {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xAC }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDY (Ldy) not implemented!");
        // Ok(())
    }
}

/// LdyZpX: LDY absolute, indexed by X
pub struct LdyZpX { }
impl Instruction for LdyZpX {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xB4 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDY (Ldy) not implemented!");
        // Ok(())
    }
}

/// LdyAbsX: LDY absolute, indexed by X
pub struct LdyAbsX { }
impl Instruction for LdyAbsX {
    fn opcode (&self) -> &'static str  { "LDY"}
    fn hexcode(&self) -> Byte { 0xBC }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode LDY (Ldy) not implemented!");
        // Ok(())
    }
}
