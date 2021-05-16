use crate::types::{ Bit, Byte, Word, CpuFlag };

/// this trait provides an object creation function
pub trait Factory {
    /// create a new instance of Self
    fn new() -> Self;
}

/// this trait is used to control the state of the cpu and its memory: reset, step, run and load data
pub trait CpuController {
    /// reset the cpu and memory to default state
    fn reset(&mut self) -> std::io::Result<()>;
    /// keep executing instructions until BRK is set
    fn run(&mut self) -> std::io::Result<()>;
    /// step one instruction
    fn step(&mut self) -> std::io::Result<()>;
    /// load a program into memory
    fn load_rom(&mut self, data: &[Byte], len: usize, target_base_address: Word) -> std::io::Result<()>;
}

/// this trait enables instructions to manipulate cpu state
pub trait VirtualCpu {
    /// get the contents of the A register
    fn get_a(&self) -> Byte;
    /// set the contents of the A register
    fn set_a(&mut self, value: Byte);
    
    /// get the contents of the X register
    fn get_x(&self) -> Byte;
    /// set the contents of the X register
    fn set_x(&mut self, value: Byte);
    
    /// get the contents of the Y register
    fn get_y(&self) -> Byte;
    /// set the contents of the Y register
    fn set_y(&mut self, value: Byte);
    
    /// get the contents of the PC (program counter)
    fn get_pc(&self) -> Word;
    /// set the contents of the PC (program counter)
    fn set_pc(&mut self, value: Word);
    
    /// set the contents of the SP (stack pointer)
    fn get_sp(&self) -> Word;
    /// set the contents of the SP (stack pointer)
    fn set_sp(&mut self, value: Word);
    
    /// get the specified CPU flag
    fn get_flag(&self, flag: CpuFlag) -> Bit;
    /// set the specified CPU flag
    fn set_flag(&mut self, flag: CpuFlag, value: Bit);
    
    /// fetch a single byte, increasing PC by 1
    fn fetch_byte(&mut self) -> std::io::Result<Byte>;
    /// fetch a single word, little endian, increasing PC by 2
    fn fetch_word(&mut self) -> std::io::Result<Word>;
    
    /// read a byte from memory at the given address without changing the program counter
    fn read_byte(&self, address: Word) -> Byte;
    /// write a byte to memory at the given address without changing the program counter
    fn write_byte(&mut self, address: Word, value: Byte);
    
    /// read a word (2 bytes, little endian) from memory at the given address without changing the program counter
    fn read_word(&self, address: Word) -> Word;
}

/// Generic CPU instruction
pub trait Instruction {
    /// the textual representation
    fn opcode(&self) -> &str;
    /// the binary opcode
    fn hexcode(&self) -> Byte;
    /// execute the opcode's logic on a virtual cpu
    fn execute(&self, cpu: &mut dyn VirtualCpu) -> std::io::Result<()>;
}

pub trait MemoryController {
    /// read a byte from memory
    fn read(&self, address: Word) -> Byte;
    /// write a byte to memory
    fn write(&mut self, address: Word, value: Byte);
    /// reset all memory locations to 0
    fn reset(&mut self);
}