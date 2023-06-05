use colored::Colorize;

pub fn elfychan_printerr(msg:&str){
    println!("{}",Colorize::bright_red(msg));
}

pub fn elfychan_printinfo(msg:&str){
    println!("{}",Colorize::bright_yellow(msg));
}

pub fn elfychan_printcyan(msg:&str){
    println!("{}",Colorize::bright_cyan(msg));
}

pub fn elfychan_printgreen(msg:&str){
    println!("{}",Colorize::bright_green(msg));
}