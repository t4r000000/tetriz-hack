use std::fs::File;
use std::io::{BufReader, Read};

use crate::Statement;

pub struct Parser;

impl Parser {
    pub fn parse(path: &str) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        let f = match File::open(path) {
            Ok(x) => x,
            _ => panic!("invalid file path"),
        };
        let mut reader = BufReader::new(f);
        reader.read_to_end(&mut buf).expect("panic at reading");

        let mut result: Vec<u8> = Vec::new();
        for bb in buf {
            result.push(bb);
        }
        result
    }

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
        );
        assert_eq!(
            Parser::parse_lines(
                r#"// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems "
// by Nisan and Schocken, MIT Press.
// File name: projects/06/add/Add.asm

// Computes R0 = 2 + 3  (R0 refers to RAM[0])

@2
D=A
@3
D=D+A
@0
M=D"#
            ),
            [
                Statement::new('A', Some(2), None, None, None),
                Statement::new('C', None, Some("A"), Some("D"), None),
                Statement::new('A', Some(3), None, None, None),
                Statement::new('C', None, Some("D+A"), Some("D"), None),
                Statement::new('A', Some(0), None, None, None),
                Statement::new('C', None, Some("D"), Some("M"), None)
            ]
        )
    }
}
