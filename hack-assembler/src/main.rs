mod parser;
mod statement;

pub use crate::parser::Parser;
pub use crate::statement::Statement;
use std::str;

fn main() {
    let src =
        parser::Parser::parse("/Users/Tarok/Developments/tetriz-hack/hack-assembler/test/Add.asm");

    let bb = str::from_utf8(&src).unwrap();

    let statements = parser::Parser::parse_lines(bb);

    for state in statements {
        println!("{:?}", state.code());
    }
}
