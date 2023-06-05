use goblin::elf::program_header::ProgramHeader;
use goblin::elf::program_header;

use crate::common::colors::elfychan_printinfo;

pub fn parse_program_header(header:ProgramHeader){
    parse_ptype(header.p_type);
    parse_pflags(header.p_flags);
    parse_poffset(header.p_offset);
    parse_vaddr(header.p_vaddr);
    parse_paddr(header.p_paddr);
    parse_pfilesz(header.p_filesz);
    parse_pmemsz(header.p_memsz);
    parse_palign(header.p_align);
}

fn parse_ptype(ptype:u32){ 
    // Segment type
    match ptype{
        0=>{
            elfychan_printinfo("Type -> NULL");
        }
        1=>{
            elfychan_printinfo("Type -> LOAD");
        }
        2=>{
            elfychan_printinfo("Type -> DYNAMIC");
        }
        3=>{
            elfychan_printinfo("Type -> INTERP");
        }
        4=>{
            elfychan_printinfo("Type -> NOTE");
        }
        5=>{
            elfychan_printinfo("Type -> SHLIB");
        }
        6=>{
            elfychan_printinfo("Type -> PHDR");
        }
        7=>{
            elfychan_printinfo("Type -> TLS");
        }
        program_header::PT_GNU_EH_FRAME=>{
            elfychan_printinfo("Type -> GNU_EH_FRAME");
        }
        program_header::PT_GNU_PROPERTY=>{
            elfychan_printinfo("Type -> GNU_PROPERTY");
        }
        program_header::PT_GNU_RELRO=>{
            elfychan_printinfo("Type -> GNU_RELRO");
        }
        program_header::PT_GNU_STACK=>{
            elfychan_printinfo("Type -> GNU_STACK");
        }
        // PT_GNU_EH_FRAME	(PT_LOOS + 0x474e550)
        // PT_GNU_STACK	(PT_LOOS + 0x474e551)
        // PT_GNU_RELRO	(PT_LOOS + 0x474e552)
        // PT_GNU_PROPERTY	(PT_LOOS + 0x474e553)

        _=>{
            elfychan_printinfo("Type -> Unknown");
        }
    }
}

fn parse_poffset(poffset:u64){
    let msg = format!("Offset -> {:#x}",poffset);
    elfychan_printinfo(msg.as_str());
}

fn parse_vaddr(pvaddr:u64){
    let msg = format!("Virtual address -> {:#x}",pvaddr);
    elfychan_printinfo(msg.as_str());
}

fn parse_paddr(ppaddr:u64){
    let msg = format!("Physical address -> {:#x}",ppaddr);
    elfychan_printinfo(msg.as_str());
}

fn parse_pfilesz(pfilesz:u64){
    let msg = format!("Size in file -> {:#x} bytes",pfilesz);
    elfychan_printinfo(msg.as_str());
}

fn parse_pmemsz(pmemsz:u64){
    let msg = format!("Size in memory -> {:#x} bytes",pmemsz);
    elfychan_printinfo(msg.as_str());
}

fn parse_palign(palign:u64){
    let msg = format!("Align -> {:#x}",palign);
    elfychan_printinfo(msg.as_str());
}

fn parse_pflags(pflags:u32){
    match pflags{
        4=>elfychan_printinfo("Flags -> R"),
        2=>elfychan_printinfo("Flags -> W"),
        1=>elfychan_printinfo("Flags -> X"),
        6=>elfychan_printinfo("Flags -> RW"),
        5=>elfychan_printinfo("Flags -> RX"),
        3=>elfychan_printinfo("Flags -> WX"),
        7=>elfychan_printinfo("Flags -> RWX"),
        _=>elfychan_printinfo("Flags -> Unknown flags")
    }
}