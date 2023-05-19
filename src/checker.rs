extern crate goblin;
use goblin::elf::Elf;
use std::{fs::File, io::Read};

pub fn check_header(file_path:&str)->bool{
    let mut file = File::open(file_path).expect("File not found");
    let mut buffer:Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read ELF");
    
    return true;
}