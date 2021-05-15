pub type Bit = u8;
pub type Byte = u8;
pub type Word = u16;
pub type Num = usize;
pub type CpuFlag = usize;

pub struct CpuFlags { }
impl CpuFlags {
    pub const CARRY: CpuFlag              = 0;
    pub const ZERO: CpuFlag               = 1;
    pub const INTERRUPTDISABLE: CpuFlag   = 2;
    pub const DECIMAL: CpuFlag            = 3;
    pub const BREAK: CpuFlag              = 4;
    pub const FUTUREUSE: CpuFlag          = 5;
    pub const OVERFLOW: CpuFlag           = 6;
    pub const NEGATIVE: CpuFlag           = 7;
}

pub trait Factory {
    fn new() -> Self;
}

pub trait VirtualCpu {
    fn reset(&mut self);
    fn run(&mut self);
    fn step(&mut self);
    fn load_rom(&mut self, data: &[Byte], base_address: Word);
    fn fetch_byte(&mut self) -> Byte;
    fn fetch_word(&mut self) -> Word;

    fn get_a(&self) -> Byte;
    fn set_a(&mut self, value: Byte);
    fn get_x(&self) -> Byte;
    fn set_x(&mut self, value: Byte);
    fn get_y(&self) -> Byte;
    fn set_y(&mut self, value: Byte);
    fn get_pc(&self) -> Word;
    fn set_pc(&mut self, value: Word);
    fn get_sp(&self) -> Word;
    fn set_sp(&mut self, value: Word);
    fn get_flag(&self, flag: CpuFlag) -> Bit;
    fn set_flag(&mut self, flag: CpuFlag, value: Bit);

    fn read_memory(&self, address: Word) -> Byte;
    fn read_memory_word(&self, address: Word) -> Word;
    fn write_memory(&mut self, address: Word, value: Byte);
}

pub trait Instruction {
    fn opcode(&self) -> &str;
    fn hexcode(&self) -> Byte;
    fn execute(&self, cpu: &mut dyn VirtualCpu);
}