use self::file::check_file;

mod header;
mod file;
pub fn initial_check(filename:&str){
    let result = check_file(filename);
    if result.0{
        println!("{}",result.1);
    }
    else{
        
    }
}