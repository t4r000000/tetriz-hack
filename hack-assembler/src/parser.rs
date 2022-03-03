use std::fs::File;
use std::io::BufReader;
use std::str::ParseBoolError;

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

    pub fn parse_lines(src: &str) -> Vec<Statement> {
        let mut result: Vec<Statement> = Vec::new();
        for line in Parser::select_effective_lines(src) {
            result.push(Parser::parse_line(line));
        }
        result
    }

    fn parse_line<'a>(line: &'a str) -> Statement<'a> {
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

        let jmp = if line_comp.len() > 1 {
            line_comp.pop()
        } else {
            None
        };

        fn wrap_str(s: &str) -> Option<&str> {
            match s.len() {
                0 => None,
                _ => Some(s),
            }
        }
        // Option<T>
        // T -> Option<T> wrap_str
        // https://hoverbear.org/blog/option-monads-in-rust/
        // Maybe Monad in Haskell, bind operator >>=

        let comp = line_comp.pop().and_then(wrap_str);

        Statement::new('C', None, comp, dest, jmp)
    }

    fn select_effective_lines<'a>(line: &'a str) -> Vec<&'a str> {
        let mut effective_lines = Vec::new();
        for ee in line.split('\n') {
            let trimed = Parser::trim_line(ee);
            if trimed.is_empty() {
                continue;
            }
            effective_lines.push(trimed)
        }
        effective_lines
    }
    // trim comment(//), blanl(\t), cariage return(\r)
    fn trim_line<'a>(line: &'a str) -> &'a str {
        let trimed_space = line.trim(); // \t trim
        let trimed_cr = trimed_space.trim_matches('\r'); // \t trim
        let trimed_comment: Vec<&str> = trimed_cr.split("//").collect();
        trimed_comment[0]
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
        assert_eq!(
            Parser::parse_line("D+A"),
            Statement::new('C', None, Some("D+A"), None, None)
        );
        assert_eq!(
            Parser::parse_line("D+A;JMP"),
            Statement::new('C', None, Some("D+A"), None, Some("JMP"))
        );
        assert_eq!(
            Parser::parse_line(";JMP"),
            Statement::new('C', None, None, None, Some("JMP"))
        );
        assert_eq!(
            Parser::select_effective_lines("M=D+A;JMP\n;JEQ"),
            ["M=D+A;JMP", ";JEQ"]
        );

        assert_eq!(
            Parser::select_effective_lines("\nM=D+A;JMP\n;JEQ"),
            ["M=D+A;JMP", ";JEQ"]
        );
        assert_eq!(
            Parser::select_effective_lines("//\nM=D+A;JMP\n;JEQ"),
            ["M=D+A;JMP", ";JEQ"]
        );
        assert_eq!(
            Parser::parse_lines("//\n@1\nM=D+A;JMP\n;JEQ"),
            [
                Statement::new('A', Some(1), None, None, None),
                Statement::new('C', None, Some("D+A"), Some("M"), Some("JMP")),
                Statement::new('C', None, None, None, Some("JEQ"))
            ]
        )
    }
}
