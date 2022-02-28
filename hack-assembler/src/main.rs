mod parser;
mod statement;

pub use crate::statement::Statement;

fn main() {
    let moji = "aaaa".chars().collect::<Vec<char>>();

    println!("Hello, world!");
}
