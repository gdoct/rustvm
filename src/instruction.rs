mod generic;
mod accum;
mod controlflow;
use crate::types::{ Byte };
use crate::traits::{ Instruction };
use crate::instruction::generic::*;
use crate::instruction::accum::*;
use crate::instruction::controlflow::*;

use std::io::{ Error, ErrorKind };

pub fn parse_opcode(opcode: Byte) -> std::result::Result<Box<dyn Instruction + 'static>, Error> {
    match opcode {
        // BRK
        0x00 => { Ok(Box::new(Brk {})) }

        // NOP
        0xea => { Ok(Box::new(Nop {})) }

        // JMP
        0x4c => { Ok(Box::new(JmpAbs {})) }
        0x6c => { Ok(Box::new(JmpInd {})) }

        // LDA
        0xa1 => { Ok(Box::new(LdaIndX {})) }
        0xb1 => { Ok(Box::new(LdaIndY {})) }
        0xa5 => { Ok(Box::new(LdaZp {})) }
        0xb5 => { Ok(Box::new(LdaZpX {})) }
        0xa9 => { Ok(Box::new(LdaImm {})) }
        0xb9 => { Ok(Box::new(LdaAbsY {})) }
        0xad => { Ok(Box::new(LdaAbs {})) }
        0xbd => { Ok(Box::new(LdaAbsX {})) }

        _ => { Err(Error::new(ErrorKind::Other, format!("opcode {} not implemented", opcode))) }
    }
}