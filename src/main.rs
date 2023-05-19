mod checker;
fn main(){
    let filename:&str = "/home/sh4dy/InfoSec/projects/elfychan/src/elfychan";
    if checker::check_header(filename){
        println!("Valid ELF header");
    }
    else{
        println!("Invalid ELF header");
    }
}