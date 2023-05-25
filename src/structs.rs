pub struct elfychan_ident{
    pub ei_class:u8,
    pub ei_data:u8,
    pub ei_version:u8,
    pub ei_osabi:u8,
    pub ei_abiversion:u8
}

pub struct elfychan_new_header{
    etype:u8,
    eversion:u8,
    eentry:u8,
    
}
impl elfychan_ident{
    pub fn parse_class(&self){
        match self.ei_class{
            1=>println!("Class -> ELF 32-bit"),
            2=>println!("Class -> ELF 64-bit"),
            _=>println!("Class -> Unknown class {}",self.ei_class)
        }
    }
    pub fn parse_data(&self){
        match self.ei_data{
            1=>println!("Encoding -> Little endian"),
            2=>println!("Encoding -> Big endian"),
            _=>println!("Encoding -> Invalid format")
        }
    }
    pub fn parse_version(&self){
        if self.ei_version<=0{
            println!("Version -> Invalid version");
        }
        else{
            println!("Version -> Current");
        }
    }
    pub fn parse_abi(&self){
        match self.ei_osabi{
            0=>println!("ABI -> ELFOSABI_SYSV"),
            1=>println!("ABI -> ELFOSABI_HPUX"),
            2=>println!("ABI -> ELFOSABI_NETBSD"),
            3=>println!("ABI -> ELFOSABI_GNU"),
            4=>println!("ABI -> ELFOSABI_HURD"),
            6=>println!("ABI -> ELFOSABI_SOLARIS"),
            7=>println!("ABI -> ELFOSABI_AIX"),
            8=>println!("ABI -> ELFOSABI_IRIX"),
            9=>println!("ABI -> ELFOSABI_FREEBSD"),
            10=>println!("ABI -> ELFOSABI_TRU64"),
            11=>println!("ABI -> ELFOSABI_MODESTO"),
            12=>println!("ABI -> ELFOSABI_OPENBSD"),
            14=>println!("ABI -> ELFOSABI_NSK"),
            64=>println!("ABI -> ELFOSABI_ARM_AEABI"),
            97=>println!("ABI -> ELFOSABI_ARM"),
            255=>println!("ABI -> ELFOSABI_STANDALONE"),
            _=>{}
        }
    }
}