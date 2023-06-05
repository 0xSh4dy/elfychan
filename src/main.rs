mod initializer;
mod checkers;
mod elfutils;
mod constants;
mod common;

fn main(){
    let filename:String = String::from("/home/sh4dy/InfoSec/projects/elfychan/src/elfychan");
    initializer::initialize(&filename);
}