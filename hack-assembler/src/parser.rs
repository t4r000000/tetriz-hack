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
        if let Some('@') = line.chars().next() {
            Statement::new(
                'A',
                Some(line.replace("@", "").parse().unwrap()),
                None,
                None,
                None,
            )
        } else {
            Parser::parse_c(line)
        }
    }

    fn parse_c(line: &str) -> Statement {
        let mut line_dest = line.split('=').rev().collect::<Vec<&str>>();

        let dest = if line_dest.len() > 1 {
            line_dest.pop()
        } else {
            None
        };
        let mut line_comp = line_dest[0].split(';').collect::<Vec<&str>>();
        println!("{:?}", line_comp);
        let jmp = if line_comp.len() > 1 {
            line_comp.pop()
        } else {
            None
        };

        let comp = line_comp.pop();
        Statement::new('C', None, comp, dest, jmp)
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
        );
        assert_eq!(
            Parser::parse_line("@50"),
            Statement::new('A', Some(50), None, None, None)
        );
        println!("{:?}", Parser::parse_line("D+A"));
        assert_eq!(
            Parser::parse_line("D+A"),
            Statement::new('C', None, Some("D+A"), None, None)
        )
    }
}
