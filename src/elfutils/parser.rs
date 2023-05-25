extern crate goblin;
extern crate colored;

use colored::Colorize;
use goblin::elf::Elf;
use goblin::elf::Header;
use crate::elfutils::elfheader::{elfychan_ident,elfychan_new_header};

pub fn parse_elf_header(buffer:Vec<u8>){
    let result: Result<Header, goblin::error::Error> = Elf::parse_header(&buffer);
    match result{
        Ok(header)=>{
            println!("{}",Colorize::bright_yellow("\n..................ELF Header.................\n"));
            let ident = header.e_ident;
            let elfy_ident = elfychan_ident{
                ei_class:ident[4],
                ei_data:ident[5],
                ei_version:ident[6],
                ei_osabi:ident[7],
                ei_abiversion:ident[8]
            };
            let elfy_new_header = elfychan_new_header{
                etype:header.e_type,
                emachine:header.e_machine,
                eversion:header.e_version,
                eentry:header.e_entry,
                ephoff:header.e_phoff,
                eshoff:header.e_shoff,
                eflags:header.e_flags,
                eehsize:header.e_ehsize,
                ephentsize:header.e_phentsize,
                ephnum:header.e_phnum,
                eshentsize:header.e_shentsize,
                eshnum:header.e_shnum,
                eshstrndx:header.e_shstrndx
            };

            elfy_ident.parse_class();
            elfy_ident.parse_data();
            elfy_ident.parse_version();
            elfy_ident.parse_abi();
            elfy_new_header.parse_e_type();
            elfy_new_header.parse_e_machine();
            elfy_new_header.parse_e_version();
            elfy_new_header.parse_entrypoint();
        },
        Err(_)=>{
            println!("Error! Failed to parse ELF header");
        }
    }
}

fn parse_e_type(etype:u8){
    match etype{
        0=>println!("Type: ET_NONE(No type)"),
        1=>println!("Type: ET_REL(Relocatable file)"),
        2=>println!("Type: ET_EXEC(Executable file)"),
        3=>println!("Type: ET_DYN(Shared object file)"),
        4=>println!("Type: ET_CORE(Core file)"),
        _=>println!("Type: Invalid")
    }
}