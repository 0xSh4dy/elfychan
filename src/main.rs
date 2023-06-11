mod checkers;
mod elfutils;
mod constants;
mod common;
use clap::{Parser};
use std::{fs::File,io::Error};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli{
    /// Path of the ELF file to be parsed
    #[arg(short,long)]
    file:String,
    /// Parse ELF headers
    #[arg(short,long)]
    elf_headers:bool,
    /// Parse program headers
    #[arg(short,long)]
    program_headers:bool,
    /// Parse section headers
    #[arg(short,long)]
    section_headers:bool
}
fn main(){
    let cli = Cli::parse();
    let elf_path = cli.file.as_str();
    let result:Result<File,Error> = checkers::file::open_file(elf_path);
    println!("{}",cli.program_headers);
    match result{
        Ok(file)=>{
            let data:Vec<u8> = checkers::file::read_file(file);
            if cli.elf_headers==true{
                elfutils::parser::parse_elf_header(&data);
            }
            else if cli.program_headers==true{
                elfutils::parser::parse_program_headers(&data);
            }
            else if cli.section_headers==true{
                elfutils::parser::parse_section_headers(&data);
            }
        },
        Err(err)=>{
            println!("{}",err);
        }
    }

}