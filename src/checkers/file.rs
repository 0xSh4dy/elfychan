use std::{fs::File,io::{Error, Read}};

pub fn open_file(filename:&str)->Result<File,Error>{
    let result: Result<File, std::io::Error> = File::open(filename);
    return result;
}

pub fn read_file(mut file:File)->Vec<u8>{
    let mut buffer:Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    return buffer;
}