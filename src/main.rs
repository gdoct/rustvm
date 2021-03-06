pub(crate) mod types;
pub(crate) mod traits;
pub(crate) mod memory;
pub(crate) mod cpu;
pub(crate) mod parser;
pub(crate) mod instructions;

#[macro_use]
extern crate clap;

use clap::App;
use std::io::{self, BufRead};
use crate::cpu::Cpu;
use crate::types::{ Byte };
use crate::traits::{ CpuController, Factory };
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>  {
    let mut vm = setup_machine()?;
    vm.reset()?;
    println!("Starting VM..");
    vm.run()?;
    println!("VM halted.");
    wait_for_enter()?;
    Ok(())
}

fn wait_for_enter() -> std::io::Result<()> {
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line)?;
    Ok(())
}

fn load_rom(filename: &str) -> std::io::Result<([Byte;0xffff], usize)> {
    let mut file = File::open(filename)?;
    let mut data : [Byte;0xffff] = [0;0xffff];
    let len = file.read(&mut data).unwrap();
    Ok((data, len))
}

fn setup_machine() -> std::io::Result<Box<dyn CpuController + 'static>> {
    let yaml = load_yaml!("app.yml");
    let command_line_args = App::from_yaml(yaml).get_matches();
    let romfilename = match command_line_args.value_of("INPUT") {
         Some(item) => {
             println!("loading rom from {}", item);
             item
         },
         None => {
             println!("loading default rom from rom.bin");
             "rom.bin"
         }
     };
    let rom = match load_rom(romfilename) {
        Ok(data) => data,
        _ => { panic!("invalid rom file"); }
    };
    let mut cpu = Cpu::new();
    cpu.load_rom(&rom.0, rom.1, 0)?;
    Ok(Box::new(cpu))
}