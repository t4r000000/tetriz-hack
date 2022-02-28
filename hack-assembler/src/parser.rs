use std::fs::File;
use std::io::BufReader;

use crate::Statement;

struct Parser;

impl Parser {
  pub fn new() -> Self {
    Parser {}
  }

  // pub fn parse_lines(path: &str) {
  //   let f = File::open(path);
  //   let reader = BufReader::new(f);

  //   for line in reader.lines() {
  //     let line = line?;
  //   }
  // }

  pub fn parse_line(line: &str) -> Statement {
    match line.chars().nth(0) {
      Some('@') => Statement::new(
        'A',
        Some(line.replace("@", "").parse().unwrap()),
        None,
        None,
        None,
      ),
      None => todo!(),
      _ => todo!(),
    }
  }
}

#[cfg(test)]
mod parser_tests {
  use super::*;
  #[test]
  fn test_ok() {
    assert_eq!(
      Parser::parse_line("@1"),
      Statement::new('A', Some(1), None, None, None)
    )
  }
}
