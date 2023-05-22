mod initializer;
mod checkers;
fn main(){
    let filename:String = String::from("elfychan");
    initializer::initialize(&filename);
}