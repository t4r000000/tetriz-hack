use std::fs::File;
use std::io::BufReader;

mod statement;

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

  pub fn parse_line(line: &str) -> statement::Statement {
    match line.chars().nth(0) {
      Some('@') => statement::Statement::new(
        'A',
        Some(line[..1].parse::<u32>().unwrap()), //銭湯を除いた文字列
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
      statement::Statement {
        state: 'A',
        value: Some(1),
        comp: None,
        dest: None,
        jump: None
      }
    )
  }
}
