use crate::constants::{ Byte, Word, Bit, VirtualCpu, CpuFlags, CpuFlag, Factory };
use crate::memory::Memory;
use crate::instruction::{parse_opcode};

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
            a : 0, x : 0, y : 0, 
            pc : 0xFFFC, sp : 0x0100,
            flags : [0; 8]
        }
    }
}

impl VirtualCpu for Cpu {
    fn reset(&mut self) {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.pc = 0xFFFC;
        self.sp = 0x0100;
        for i in 0..7 { self.flags[i] = 0 }
        self.memory.reset();
    }

    fn run(&mut self) {
        while self.flags[CpuFlags::BREAK] != 1 {
            self.step();
        } 
    }

    fn step(&mut self) {
        let opcode = self.fetch_byte();
        match parse_opcode(opcode) {
            Ok(instr) => { println!("{}", instr.opcode()); instr.execute(self) },
            Err(_) => { println!("opcode not implemented: {}", opcode); panic!("opcode not implemented") }
        };    
    }

    fn load_rom(&mut self, data: &[Byte], base_address: Word) {
        let max = data.len() - 1;
        let baseaddress: usize = base_address as usize;
        if max + baseaddress > 0xffff { panic!("program does not fit in available memory") }
        for index in 0..max {
            self.memory.write((baseaddress + index) as Word, data[index]);
        }
    }

    fn fetch_byte(&mut self) -> Byte {
        let byte = self.memory.read(self.pc);
        if self.pc == 0xffff {
            self.flags[CpuFlags::BREAK] = 1;
        } else {
            self.pc += 1;
        }
        byte
    }

    fn fetch_word(&mut self) -> Word { create_word((self.fetch_byte(), self.fetch_byte())) }
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
    fn read_memory(&self, address: Word) -> Byte { self.memory.read(address) }
    fn read_memory_word(&self, address: Word) -> Word { create_word((self.read_memory(address), self.read_memory(address + 1)))}
    fn write_memory(&mut self, address: Word, value: Byte) { self.memory.write(address, value) }
}