use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// RolZp: ROL zeropage
pub struct RolZp { }
impl Instruction for RolZp {
    fn opcode (&self) -> &'static str  { "ROL"}
    fn hexcode(&self) -> Byte { 0x26 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROL (Rol) not implemented!");
        // Ok(())
    }
}

/// Rol: ROL 
pub struct Rol { }
impl Instruction for Rol {
    fn opcode (&self) -> &'static str  { "ROL"}
    fn hexcode(&self) -> Byte { 0x2A }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROL (Rol) not implemented!");
        // Ok(())
    }
}

/// RolAbs: ROL absolute
pub struct RolAbs { }
impl Instruction for RolAbs {
    fn opcode (&self) -> &'static str  { "ROL"}
    fn hexcode(&self) -> Byte { 0x2E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROL (Rol) not implemented!");
        // Ok(())
    }
}

/// RolZpX: ROL absolute, indexed by X
pub struct RolZpX { }
impl Instruction for RolZpX {
    fn opcode (&self) -> &'static str  { "ROL"}
    fn hexcode(&self) -> Byte { 0x36 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROL (Rol) not implemented!");
        // Ok(())
    }
}

/// RolAbsX: ROL absolute, indexed by X
pub struct RolAbsX { }
impl Instruction for RolAbsX {
    fn opcode (&self) -> &'static str  { "ROL"}
    fn hexcode(&self) -> Byte { 0x3E }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ROL (Rol) not implemented!");
        // Ok(())
    }
}
