use std::fs::File;

pub fn check_file(filename:&str)->(bool,String){
    let result = File::open(filename);
    let mut error_status =false;
    let mut error_message = String::new();
    match result{
        Ok(_)=>{
        }
        Err(_)=>{
            error_status = true;
            error_message = format!("Failed to open {}",filename);
        }
    }
    return (error_status,error_message);
}