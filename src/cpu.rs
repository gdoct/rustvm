use crate::types::{ Byte, Word, Bit, CpuFlags, CpuFlag };
use crate::traits::{ VirtualCpu, CpuController, Factory, MemoryController };
use crate::memory::Memory;
use crate::instruction::{parse_opcode};
use std::io::{Error, ErrorKind};

fn create_word(bytes: (Byte, Byte)) -> Word { ((bytes.0 as Word) << 8) | bytes.1 as Word }

pub struct Cpu {
    memory: Memory,
    a : Byte,                   // A register
    x : Byte,                   // X register
    y : Byte,                   // Y register
    pc : Word,                  // Program counter
    sp : Word,                  // Stack pointer
    flags : [Bit; 8]            // Cpu flags
}

impl Factory for Cpu {
    fn new() -> Self {
        Self { 
            memory: Memory::new(),
            a : 0, 
            x : 0, 
            y : 0, 
            pc : 0xFFFC, 
            sp : 0x0100,
            flags : [false; 8]
        }
    }
}

impl CpuController for Cpu {
    fn reset(&mut self) -> std::io::Result<()> {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        for i in 0..7 { self.flags[i] = false }
        self.memory.reset();
        Ok(())
    }

    fn run(&mut self) -> std::io::Result<()> {
        while !self.flags[CpuFlags::BREAK] {
            self.step()?;
        } 
        Ok(())
    }

    fn step(&mut self) -> std::io::Result<()> {
        let opcode = self.fetch_byte()?;
        let instr = parse_opcode(opcode)?;
        instr.execute(self)?;
        Ok(())
    }

    fn load_rom(&mut self, data: &[Byte], len: usize, base_address: Word) -> std::io::Result<()> {
        let datalen = data.len();
        if len > datalen || len == 0 || datalen==0 { return Err(Error::new(ErrorKind::Other, "invalid program, length")) }
        let max = len - 1;
        let baseaddress: usize = base_address as usize;
        if max + baseaddress > 0xffff { return Err(Error::new(ErrorKind::Other, "program does not fit in memory")) }
        for index in 0..max {
            self.memory.write((baseaddress + index) as Word, data[index]);
        }
        Ok(())
    }
}

impl VirtualCpu for Cpu {
    fn fetch_byte(&mut self) -> std::io::Result<Byte> {
        let byte = self.memory.read(self.pc);
        if self.pc == 0xffff {
            self.flags[CpuFlags::BREAK] = true;
        } else {
            self.pc += 1;
        }
        Ok(byte)
    }

    fn fetch_word(&mut self) -> std::io::Result<Word> { Ok(create_word((self.fetch_byte()?, self.fetch_byte()?))) }
    fn get_a(&self) -> Byte { self.a }
    fn set_a(&mut self, value: Byte) { self.a = value }
    fn get_x(&self) -> Byte{ self.x }
    fn set_x(&mut self, value: Byte) { self.x = value }
    fn get_y(&self) -> Byte{ self.y }
    fn set_y(&mut self, value: Byte) { self.y = value }
    fn get_pc(&self) -> Word{ self.pc }
    fn set_pc(&mut self, value: Word) { self.pc = value }
    fn get_sp(&self) -> Word{ self.sp }
    fn set_sp(&mut self, value: Word) { self.sp = value }
    fn get_flag(&self, flag: CpuFlag) -> Bit { self.flags[flag] }
    fn set_flag(&mut self, flag: CpuFlag, value: Bit) { self.flags[flag] = value }
    fn read_byte(&self, address: Word) -> Byte { self.memory.read(address) }
    fn read_word(&self, address: Word) -> Word { create_word((self.read_byte(address), self.read_byte(address + 1)))}
    fn write_byte(&mut self, address: Word, value: Byte) { self.memory.write(address, value) }
}