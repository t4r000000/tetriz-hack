mod parser;
mod statement;

pub use crate::statement::Statement;

fn main() {
    // split =,;
    let mut moji: Vec<&str> = "A=D;JMP".split('=').rev().collect();
    println!("{}", moji.len());
    let mo = moji.pop().unwrap();
    
    println!("{:?}", mo);
}
