use crate::types::{ Byte, Word, Num };
use crate::traits::{Factory, MemoryController};

const SIZE: Num = 0xffff;

#[derive(Debug)]
pub struct Memory {
    data: [Byte; SIZE+1]
}

impl Factory for Memory {
    fn new() -> Self {
        Memory { data: [0;SIZE+1] }
    }
}

impl MemoryController for Memory {
    fn read(&self, address: Word) -> Byte {
        self.data[address as usize]
    }

    fn write(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    fn reset(&mut self) {
        for i in 0..(SIZE) {
            self.data[i] = 0;
        }
    }
}