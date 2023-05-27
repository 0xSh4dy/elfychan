use crate::checkers;
use crate::elfutils;
use std::{fs::File,io::Error};

pub fn initialize(filename:&str){
    if !checkers::initial_check(filename){ // if it doesn't return any error
        let result:Result<File,Error> = checkers::file::open_file(filename);
        match result{
            Ok(file)=>{
                let buffer:Vec<u8> = checkers::file::read_file(file);
                elfutils::parser::parse_elf_header(&buffer);
                elfutils::parser::parse_program_headers(&buffer);
            },
            Err(_)=>{

            }
        }
    }
    else{
        println!("Given file is not an ELF");
    }
}