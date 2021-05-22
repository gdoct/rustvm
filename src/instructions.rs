mod adc;
mod and;
mod asl;
mod bcc;
mod bcs;
mod beq;
mod bit;
mod bmi;
mod bne;
mod bpl;
mod brk;
mod bvc;
mod bvs;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dec;
mod dex;
mod dey;
mod eor;
mod inc;
mod inx;
mod iny;
mod jmp;
mod jsr;
mod lda;
mod ldx;
mod ldy;
mod lsr;
mod nop;
mod ora;
mod pha;
mod php;
mod pla;
mod plp;
mod rol;
mod ror;
mod rti;
mod rts;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

use std::io::{ Error, ErrorKind };
use crate::types::{ Byte };
use crate::traits::{ Instruction };
use crate::instructions::adc::*;
use crate::instructions::and::*;
use crate::instructions::asl::*;
use crate::instructions::bcc::*;
use crate::instructions::bcs::*;
use crate::instructions::beq::*;
use crate::instructions::bit::*;
use crate::instructions::bmi::*;
use crate::instructions::bne::*;
use crate::instructions::bpl::*;
use crate::instructions::brk::*;
use crate::instructions::bvc::*;
use crate::instructions::bvs::*;
use crate::instructions::clc::*;
use crate::instructions::cld::*;
use crate::instructions::cli::*;
use crate::instructions::clv::*;
use crate::instructions::cmp::*;
use crate::instructions::cpx::*;
use crate::instructions::cpy::*;
use crate::instructions::dec::*;
use crate::instructions::dex::*;
use crate::instructions::dey::*;
use crate::instructions::eor::*;
use crate::instructions::inc::*;
use crate::instructions::inx::*;
use crate::instructions::iny::*;
use crate::instructions::jmp::*;
use crate::instructions::jsr::*;
use crate::instructions::lda::*;
use crate::instructions::ldx::*;
use crate::instructions::ldy::*;
use crate::instructions::lsr::*;
use crate::instructions::nop::*;
use crate::instructions::ora::*;
use crate::instructions::pha::*;
use crate::instructions::php::*;
use crate::instructions::pla::*;
use crate::instructions::plp::*;
use crate::instructions::rol::*;
use crate::instructions::ror::*;
use crate::instructions::rti::*;
use crate::instructions::rts::*;
use crate::instructions::sbc::*;
use crate::instructions::sec::*;
use crate::instructions::sed::*;
use crate::instructions::sei::*;
use crate::instructions::sta::*;
use crate::instructions::stx::*;
use crate::instructions::sty::*;
use crate::instructions::tax::*;
use crate::instructions::tay::*;
use crate::instructions::tsx::*;
use crate::instructions::txa::*;
use crate::instructions::txs::*;
use crate::instructions::tya::*;

pub fn parse_opcode(opcode: Byte) -> std::result::Result<Box<dyn Instruction + 'static>, Error> {
    match opcode {

        // ADC
        0x61 => { Ok(Box::new(AdcIndX {})) }
        0x65 => { Ok(Box::new(AdcZp {})) }
        0x69 => { Ok(Box::new(AdcImm {})) }
        0x6D => { Ok(Box::new(AdcAbs {})) }
        0x71 => { Ok(Box::new(AdcIndY {})) }
        0x75 => { Ok(Box::new(AdcZpX {})) }
        0x79 => { Ok(Box::new(AdcAbsY {})) }
        0x7D => { Ok(Box::new(AdcAbsX {})) }

        // AND
        0x21 => { Ok(Box::new(AndIndX {})) }
        0x25 => { Ok(Box::new(AndZp {})) }
        0x29 => { Ok(Box::new(AndImm {})) }
        0x2D => { Ok(Box::new(AndAbs {})) }
        0x31 => { Ok(Box::new(AndIndY {})) }
        0x35 => { Ok(Box::new(AndZpX {})) }
        0x39 => { Ok(Box::new(AndAbsY {})) }
        0x3D => { Ok(Box::new(AndAbsX {})) }

        // ASL
        0x06 => { Ok(Box::new(AslZp {})) }
        0x0A => { Ok(Box::new(Asl {})) }
        0x0E => { Ok(Box::new(AslAbs {})) }
        0x16 => { Ok(Box::new(AslZpX {})) }
        0x1E => { Ok(Box::new(AslAbsX {})) }

        // BCC
        0x90 => { Ok(Box::new(BccRel {})) }

        // BCS
        0xB0 => { Ok(Box::new(BcsRel {})) }

        // BEQ
        0xF0 => { Ok(Box::new(BeqRel {})) }

        // BIT
        0x24 => { Ok(Box::new(BitZp {})) }
        0x2C => { Ok(Box::new(BitAbs {})) }

        // BMI
        0x30 => { Ok(Box::new(BmiRel {})) }

        // BNE
        0xD0 => { Ok(Box::new(BneRel {})) }

        // BPL
        0x10 => { Ok(Box::new(BplRel {})) }

        // BRK
        0x00 => { Ok(Box::new(Brk {})) }

        // BVC
        0x50 => { Ok(Box::new(BvcRel {})) }

        // BVS
        0x70 => { Ok(Box::new(BvsRel {})) }

        // CLC
        0x18 => { Ok(Box::new(Clc {})) }

        // CLD
        0xD8 => { Ok(Box::new(Cld {})) }

        // CLI
        0x58 => { Ok(Box::new(Cli {})) }

        // CLV
        0xB8 => { Ok(Box::new(Clv {})) }

        // CMP
        0xC1 => { Ok(Box::new(CmpIndX {})) }
        0xC5 => { Ok(Box::new(CmpZp {})) }
        0xC9 => { Ok(Box::new(CmpImm {})) }
        0xCD => { Ok(Box::new(CmpAbs {})) }
        0xD1 => { Ok(Box::new(CmpIndY {})) }
        0xD5 => { Ok(Box::new(CmpZpX {})) }
        0xD9 => { Ok(Box::new(CmpAbsY {})) }
        0xDD => { Ok(Box::new(CmpAbsX {})) }

        // CPX
        0xE0 => { Ok(Box::new(CpxImm {})) }
        0xE4 => { Ok(Box::new(CpxZp {})) }
        0xEC => { Ok(Box::new(CpxAbs {})) }

        // CPY
        0xC0 => { Ok(Box::new(CpyImm {})) }
        0xC4 => { Ok(Box::new(CpyZp {})) }
        0xCC => { Ok(Box::new(CpyAbs {})) }

        // DEC
        0xC6 => { Ok(Box::new(DecZp {})) }
        0xCE => { Ok(Box::new(DecAbs {})) }
        0xD6 => { Ok(Box::new(DecZpX {})) }
        0xDE => { Ok(Box::new(DecAbsX {})) }

        // DEX
        0xCA => { Ok(Box::new(Dex {})) }

        // DEY
        0x88 => { Ok(Box::new(Dey {})) }

        // EOR
        0x41 => { Ok(Box::new(EorIndX {})) }
        0x45 => { Ok(Box::new(EorZp {})) }
        0x49 => { Ok(Box::new(EorImm {})) }
        0x4D => { Ok(Box::new(EorAbs {})) }
        0x51 => { Ok(Box::new(EorIndY {})) }
        0x55 => { Ok(Box::new(EorZpX {})) }
        0x59 => { Ok(Box::new(EorAbsY {})) }
        0x5D => { Ok(Box::new(EorAbsX {})) }

        // INC
        0xE6 => { Ok(Box::new(IncZp {})) }
        0xEE => { Ok(Box::new(IncAbs {})) }
        0xF6 => { Ok(Box::new(IncZpX {})) }
        0xFE => { Ok(Box::new(IncAbsX {})) }

        // INX
        0xE8 => { Ok(Box::new(Inx {})) }

        // INY
        0xC8 => { Ok(Box::new(Iny {})) }

        // JMP
        0x4C => { Ok(Box::new(JmpAbs {})) }
        0x6C => { Ok(Box::new(JmpInd {})) }

        // JSR
        0x20 => { Ok(Box::new(JsrAbs {})) }

        // LDA
        0xA1 => { Ok(Box::new(LdaIndX {})) }
        0xA5 => { Ok(Box::new(LdaZp {})) }
        0xA9 => { Ok(Box::new(LdaImm {})) }
        0xAD => { Ok(Box::new(LdaAbs {})) }
        0xB1 => { Ok(Box::new(LdaIndY {})) }
        0xB5 => { Ok(Box::new(LdaZpX {})) }
        0xB9 => { Ok(Box::new(LdaAbsY {})) }
        0xBD => { Ok(Box::new(LdaAbsX {})) }

        // LDX
        0xA2 => { Ok(Box::new(LdxImm {})) }
        0xA6 => { Ok(Box::new(LdxZp {})) }
        0xAE => { Ok(Box::new(LdxAbs {})) }
        0xB6 => { Ok(Box::new(LdxZpY {})) }
        0xBE => { Ok(Box::new(LdxAbsY {})) }

        // LDY
        0xA0 => { Ok(Box::new(LdyImm {})) }
        0xA4 => { Ok(Box::new(LdyZp {})) }
        0xAC => { Ok(Box::new(LdyAbs {})) }
        0xB4 => { Ok(Box::new(LdyZpX {})) }
        0xBC => { Ok(Box::new(LdyAbsX {})) }

        // LSR
        0x46 => { Ok(Box::new(LsrZp {})) }
        0x4A => { Ok(Box::new(Lsr {})) }
        0x4E => { Ok(Box::new(LsrAbs {})) }
        0x56 => { Ok(Box::new(LsrZpX {})) }
        0x5E => { Ok(Box::new(LsrAbsX {})) }

        // NOP
        0xEA => { Ok(Box::new(Nop {})) }

        // ORA
        0x01 => { Ok(Box::new(OraIndX {})) }
        0x05 => { Ok(Box::new(OraZp {})) }
        0x09 => { Ok(Box::new(OraImm {})) }
        0x0D => { Ok(Box::new(OraAbs {})) }
        0x11 => { Ok(Box::new(OraIndY {})) }
        0x15 => { Ok(Box::new(OraZpX {})) }
        0x19 => { Ok(Box::new(OraAbsY {})) }
        0x1D => { Ok(Box::new(OraAbsX {})) }

        // PHA
        0x48 => { Ok(Box::new(Pha {})) }

        // PHP
        0x08 => { Ok(Box::new(Php {})) }

        // PLA
        0x68 => { Ok(Box::new(Pla {})) }

        // PLP
        0x28 => { Ok(Box::new(Plp {})) }

        // ROL
        0x26 => { Ok(Box::new(RolZp {})) }
        0x2A => { Ok(Box::new(Rol {})) }
        0x2E => { Ok(Box::new(RolAbs {})) }
        0x36 => { Ok(Box::new(RolZpX {})) }
        0x3E => { Ok(Box::new(RolAbsX {})) }

        // ROR
        0x66 => { Ok(Box::new(RorZp {})) }
        0x6A => { Ok(Box::new(Ror {})) }
        0x6E => { Ok(Box::new(RorAbs {})) }
        0x76 => { Ok(Box::new(RorZpX {})) }
        0x7E => { Ok(Box::new(RorAbsX {})) }

        // RTI
        0x40 => { Ok(Box::new(Rti {})) }

        // RTS
        0x60 => { Ok(Box::new(Rts {})) }

        // SBC
        0xE1 => { Ok(Box::new(SbcIndX {})) }
        0xE5 => { Ok(Box::new(SbcZp {})) }
        0xE9 => { Ok(Box::new(SbcImm {})) }
        0xED => { Ok(Box::new(SbcAbs {})) }
        0xF1 => { Ok(Box::new(SbcIndY {})) }
        0xF5 => { Ok(Box::new(SbcZpX {})) }
        0xF9 => { Ok(Box::new(SbcAbsY {})) }
        0xFD => { Ok(Box::new(SbcAbsX {})) }

        // SEC
        0x38 => { Ok(Box::new(Sec {})) }

        // SED
        0xF8 => { Ok(Box::new(Sed {})) }

        // SEI
        0x78 => { Ok(Box::new(Sei {})) }

        // STA
        0x81 => { Ok(Box::new(StaIndX {})) }
        0x85 => { Ok(Box::new(StaZp {})) }
        0x8D => { Ok(Box::new(StaAbs {})) }
        0x91 => { Ok(Box::new(StaIndY {})) }
        0x95 => { Ok(Box::new(StaZpX {})) }
        0x99 => { Ok(Box::new(StaAbsY {})) }
        0x9D => { Ok(Box::new(StaAbsX {})) }

        // STX
        0x86 => { Ok(Box::new(StxZp {})) }
        0x8E => { Ok(Box::new(StxAbs {})) }
        0x96 => { Ok(Box::new(StxZpY {})) }

        // STY
        0x84 => { Ok(Box::new(StyZp {})) }
        0x8C => { Ok(Box::new(StyAbs {})) }
        0x94 => { Ok(Box::new(StyZpX {})) }

        // TAX
        0xAA => { Ok(Box::new(Tax {})) }

        // TAY
        0xA8 => { Ok(Box::new(Tay {})) }

        // TSX
        0xBA => { Ok(Box::new(Tsx {})) }

        // TXA
        0x8A => { Ok(Box::new(Txa {})) }

        // TXS
        0x9A => { Ok(Box::new(Txs {})) }

        // TYA
        0x98 => { Ok(Box::new(Tya {})) }

        _ => { Err(Error::new(ErrorKind::Other, format!("opcode { } not implemented", opcode))) }
    }
}
