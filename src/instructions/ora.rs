use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// OraIndX: ORA indirect, indexed by X
pub struct OraIndX { }
impl Instruction for OraIndX {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x01 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraZp: ORA zeropage
pub struct OraZp { }
impl Instruction for OraZp {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x05 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraImm: ORA immediate
pub struct OraImm { }
impl Instruction for OraImm {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x09 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraAbs: ORA absolute
pub struct OraAbs { }
impl Instruction for OraAbs {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x0D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraIndY: ORA indirect, indexed by Y
pub struct OraIndY { }
impl Instruction for OraIndY {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x11 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraZpX: ORA absolute, indexed by X
pub struct OraZpX { }
impl Instruction for OraZpX {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x15 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraAbsY: ORA absolute, indexed by Y
pub struct OraAbsY { }
impl Instruction for OraAbsY {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x19 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}

/// OraAbsX: ORA absolute, indexed by X
pub struct OraAbsX { }
impl Instruction for OraAbsX {
    fn opcode (&self) -> &'static str  { "ORA"}
    fn hexcode(&self) -> Byte { 0x1D }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode ORA (Ora) not implemented!");
        // Ok(())
    }
}
