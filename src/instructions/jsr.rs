use crate::traits::{ Instruction, VirtualCpu };
use crate::types::{ Byte };

/// JsrAbs: JSR absolute
pub struct JsrAbs { }
impl Instruction for JsrAbs {
    fn opcode (&self) -> &'static str  { "JSR"}
    fn hexcode(&self) -> Byte { 0x20 }
    fn execute(&self, _cpu: &mut dyn VirtualCpu) -> std::io::Result<()> {
        panic!("opcode JSR (Jsr) not implemented!");
        // Ok(())
    }
}
