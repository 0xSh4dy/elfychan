use std::{fs::File, io::Error};

pub mod file;
pub mod header;

pub fn initial_check(filename: &str)->bool {
    let result: Result<File, Error> = file::open_file(filename);
    let mut error_status:bool = false;
    match result {
        Ok(file) => {
            let buffer:Vec<u8> = file::read_file(file);
            let magic_bytes_option = buffer.get(0..16);
            match magic_bytes_option{
                Some(magic_bytes)=>{
                    if header::check_magic_header(magic_bytes){
                        error_status = false;
                    }
                    else{
                        error_status = true;
                    }
                },
                None=>{
                    error_status = false;
                }
            }
        }
        Err(_) => {
            error_status = true;
        }
    }
    return error_status
}
