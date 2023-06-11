use goblin::elf::section_header::SectionHeader;
use goblin::elf::section_header;

use crate::common::colors::elfychan_printcyan;

pub fn parse_section_header(header:SectionHeader){
    parse_shname(header.sh_name);
}

fn parse_shname(shname:usize){
    let msg = format!("Index -> {}",shname);
    elfychan_printcyan(msg.as_str());
}