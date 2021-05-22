use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte, Word };

pub struct StaZp { }
 impl Instruction for StaZp {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x85 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_byte()? as Word;
        let val = cpu.get_a();
        cpu.write_byte(addr, val);
        Ok(())
    }
 }

 pub struct StaZpX { }
 impl Instruction for StaZpX {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x95 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let mut addr = (cpu.fetch_byte()? + cpu.get_x()) as Word;
        if addr > 0x00ff { addr -= 0x00ff }
        let val = cpu.get_a();
        cpu.write_byte(addr, val);
        Ok(())
    }
 }

 pub struct StaAbs { }
 impl Instruction for StaAbs {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x8D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()?;
        let val = cpu.get_a();
        cpu.write_byte(addr, val);
        Ok(())
    }
 }

 pub struct StaAbsX { }
 impl Instruction for StaAbsX {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x9D }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()? + cpu.get_x() as Word;
        let val = cpu.get_a();
        cpu.write_byte(addr, val);
        Ok(())
    }
 }

 pub struct StaAbsY { }
 impl Instruction for StaAbsY {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x99 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr = cpu.fetch_word()? + cpu.get_y() as Word;
        let val = cpu.get_a();
        cpu.write_byte(addr, val);
        Ok(())
    }
 }

 pub struct StaIndX {}
impl Instruction for StaIndX {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x81 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_in_zp = cpu.fetch_byte()? as Word;
        let mut addr_plus_x = addr_in_zp + cpu.get_x() as Word;
        if addr_plus_x > 0xff {
            addr_plus_x -= 0xff;
        }
        let a = cpu.get_a();
        cpu.write_byte(addr_plus_x, a);
        Ok(())
    }
}

pub struct StaIndY {}
impl Instruction for StaIndY {
    fn opcode (&self) -> &'static str  { "STA"}
    fn hexcode (&self) -> Byte { 0x91 }
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        let addr_in_zp = cpu.fetch_byte()? as Word;
        let mut addr_plus_y = addr_in_zp + cpu.get_y() as Word;
        if addr_plus_y > 0xff {
            addr_plus_y -= 0xff;
        }
        let a = cpu.get_a();
        cpu.write_byte(addr_plus_y, a);
        Ok(())
    }
}
