extern crate colored;
extern crate goblin;
use crate::common::colors::{elfychan_printerr, elfychan_printcyan, elfychan_printgreen, elfychan_printinfo};
use crate::elfutils::{
    elfheader::{elfychan_ident, elfychan_new_header},
    progheaders::parse_program_header,
};
use colored::Colorize;
use goblin::elf::program_header::ProgramHeader;
use goblin::elf::section_header::SectionHeader;
use goblin::elf::{Elf};
use goblin::elf::Header;
use std::mem;

use super::secheaders;

pub fn parse_elf_header(buffer: &Vec<u8>) {
    let result: Result<Header, goblin::error::Error> = Elf::parse_header(&buffer);
    match result {
        Ok(header) => {
            println!(
                "{}",
                Colorize::bright_yellow("\n..................ELF Header.................\n")
            );
            let ident = header.e_ident;
            let elfy_ident = elfychan_ident {
                ei_class: ident[4],
                ei_data: ident[5],
                ei_version: ident[6],
                ei_osabi: ident[7],
                ei_abiversion: ident[8],
            };
            let elfy_new_header = elfychan_new_header {
                etype: header.e_type,
                emachine: header.e_machine,
                eversion: header.e_version,
                eentry: header.e_entry,
                ephoff: header.e_phoff,
                eshoff: header.e_shoff,
                eflags: header.e_flags,
                eehsize: header.e_ehsize,
                ephentsize: header.e_phentsize,
                ephnum: header.e_phnum,
                eshentsize: header.e_shentsize,
                eshnum: header.e_shnum,
                eshstrndx: header.e_shstrndx,
            };
            print_magic(buffer);
            elfy_ident.parse_class();
            elfy_ident.parse_data();
            elfy_ident.parse_version();
            elfy_ident.parse_abi();
            elfy_new_header.parse_e_type();
            elfy_new_header.parse_e_machine();
            elfy_new_header.parse_e_version();
            elfy_new_header.parse_entrypoint();
            elfy_new_header.parse_ephoff();
            elfy_new_header.parse_eshoff();
            elfy_new_header.parse_eflags();
            elfy_new_header.parse_ehsize();
            elfy_new_header.parse_ephentsize();
            elfy_new_header.parse_ephnum();
            elfy_new_header.parse_eshsize();
            elfy_new_header.parse_eshnum();
            elfy_new_header.parse_eshstrndx();
        }
        Err(_) => {
            println!("Error! Failed to parse ELF header");
        }
    }
}

fn get_progheader_info(buffer: &Vec<u8>) -> (u16, u64, bool) {
    let result = Elf::parse_header(buffer);
    if result.is_ok() {
        let val = result.unwrap();
        return (val.e_phnum, val.e_phoff, false);
    }
    return (0, 0, true);
}

fn get_secheader_info(buffer:&Vec<u8>)->(u16,u64,bool){
    let result = Elf::parse_header(buffer);
    if result.is_ok(){
        let val = result.unwrap();
        return (val.e_shnum,val.e_shoff,false);
    }
    return (0,0,true);
}

pub fn parse_program_headers(buffer: &Vec<u8>) {
    let container = goblin::container::CONTAINER;
    let ctx = goblin::container::Ctx::new(container, goblin::container::Endian::Little);
    let ph_info = get_progheader_info(buffer);
    let phnum = ph_info.0;
    let phoff = ph_info.1;
    let err = ph_info.2;
    if err {
        elfychan_printerr("Failed to parse ELF header");
        return;
    }
    let result = ProgramHeader::parse(buffer, phoff as usize, phnum as usize, ctx);
    elfychan_printgreen("-----------Program Headers-------------");
    match result {
        Ok(headers) => {
            for header in headers {
                elfychan_printcyan("--------------------------------");
                parse_program_header(header);
            }
        }
        Err(_) => {
            elfychan_printerr("Failed to parse program headers");
        }
    }
}

pub fn parse_section_headers(buffer:&Vec<u8>){
    let container = goblin::container::CONTAINER;
    let ctx = goblin::container::Ctx::new(container, goblin::container::Endian::Little);
    let sh_info = get_secheader_info(buffer);
    let shnum = sh_info.0;
    let shoff = sh_info.1;
    let err = sh_info.2;
    if err{
        elfychan_printerr("Failed to parse ELF header");
    }
    let result = SectionHeader::parse(buffer, shoff as usize, shnum as usize, ctx);
    elfychan_printgreen("-----------Section Headers-------------");
    match result{
        Ok(headers)=>{
            for header in headers{
                elfychan_printinfo("--------------------------------");
                secheaders::parse_section_header(header);
            }
        },
        Err(_)=>{
            elfychan_printerr("Failed to parse section headers");
        }
    }
}
fn print_magic(buffer: &Vec<u8>) {
    print!("Magic -> ");
    for i in 0..16 {
        print!("{:#x} ", buffer[i]);
    }
    println!("");
}
