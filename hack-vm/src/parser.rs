use std::io::{BufRead, BufReader, Read};

enum CommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}
struct Commands {
    cmds: Vec<Vec<String>>,
    current_idx: usize,
}

trait Parser {
    fn new<R: Read>(f: BufReader<R>) -> Self;
    fn has_more_commands(&self) -> bool;
    fn advance(&mut self) {}
    // fn command_type() -> String;
    // fn arg1() -> String;
    // fn arg2() -> i32;
}

impl Parser for Commands {
    fn new<R: Read>(f: BufReader<R>) -> Commands {
        let mut cmds: Vec<Vec<String>> = vec![vec!["".to_string()]];
        for line in f.lines() {
            match line {
                Ok(line) => {
                    let cmd: Vec<String> = line.split(' ').map(str::to_string).collect();
                    cmds.push(cmd)
                }
                Err(e) => panic!("panic happened while reading buff, {}", e),
            }
        }
        Commands {
            cmds,
            current_idx: 0,
        }
    }
    fn has_more_commands(&self) -> bool {
        self.current_idx < self.cmds.len()
    }

    fn advance(&mut self) {
        self.current_idx += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Commands, Parser};
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_new_parser() {
        let command = Cursor::new("push constant 7\npush constant 8");
        let readio = BufReader::new(command);
        let commands = Commands::new(readio);
        let result = vec![
            vec![""],
            vec!["push", "constant", "7"],
            vec!["push", "constant", "8"],
        ];
        assert_eq!(result, commands.cmds);
    }

    #[test]
    fn test_has_more_cmds() {
        let command = Cursor::new("push constant 7\n");
        let readio = BufReader::new(command);
        let mut commands = Commands::new(readio);
        let result_true = commands.has_more_commands();
        assert!(result_true);
        commands.advance();
        let result_true = commands.has_more_commands();
        assert!(result_true);
        commands.advance();
        let result_false = commands.has_more_commands();
        assert!(!result_false);
    }

    fn test_cmdstype() {
        let command = Cursor::new("push constant 7\n");
        let readio = BufReader::new(command);
        let mut commands = Commands::new(readio);
        let result_true = commands.has_more_commands();
        assert_eq!(result_true, true);
        commands.advance();
        let result_false = commands.has_more_commands();
        assert_eq!(result_false, false);
    }
}
