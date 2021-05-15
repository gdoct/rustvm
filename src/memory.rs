use crate::constants::{ Byte, Word, Num, Factory };

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

impl Memory {
    pub fn read(&self, address: Word) -> Byte {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: Word, value: Byte) {
        self.data[address as usize] = value;
    }

    pub fn reset(&mut self) {
        for i in 0..(SIZE) {
            self.data[i] = 0;
        }
    }
}