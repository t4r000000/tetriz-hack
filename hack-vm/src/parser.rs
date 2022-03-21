use std::io::{BufRead, BufReader, Read};

#[derive(PartialEq, Debug)]
pub enum CommandType {
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
    fn command_type(&self) -> CommandType;
    fn arg1(&self) -> &str;
    fn arg2(&self) -> i32;
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
    fn command_type(&self) -> CommandType {
        match self.cmds[self.current_idx][0].as_str() {
            "push" => CommandType::C_PUSH,
            "add" => CommandType::C_ARITHMETIC,
            _ => panic!("undefined command type case"),
        }
    }

    fn arg1(&self) -> &str {
        match self.command_type() {
            CommandType::C_ARITHMETIC => self.cmds[self.current_idx][0].as_str(),
            CommandType::C_PUSH => self.cmds[self.current_idx][1].as_str(),
            _ => panic!("undefined command type at arg1"),
        }
    }

    fn arg2(&self) -> i32 {
        match self.command_type() {
            CommandType::C_PUSH => self.cmds[self.current_idx][2]
                .parse()
                .expect("invalid arg2,not int"),
            _ => panic!("invalid command type at arg2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{CommandType, Commands, Parser};
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

    #[test]
    fn test_cmdstype_push() {
        let command = BufReader::new(Cursor::new("push constant 7\n"));
        let mut cmds = Commands::new(command);
        cmds.advance();
        let arthmethtic = cmds.command_type();
        assert_eq!(CommandType::C_PUSH, arthmethtic);
    }

    #[test]
    fn test_cmdstype_add() {
        let command = BufReader::new(Cursor::new("add\n"));
        let mut cmds = Commands::new(command);
        cmds.advance();
        let arthmethtic = cmds.command_type();
        assert_eq!(CommandType::C_ARITHMETIC, arthmethtic);
    }
    #[test]
    fn test_cmdstype_simple_add() {
        let command = BufReader::new(Cursor::new("push constant 8\npush constant 7\nadd\n"));
        let mut cmds = Commands::new(command);
        cmds.advance();
        let arthmethtic = cmds.command_type();
        let arg1 = cmds.arg1();
        let arg2 = cmds.arg2();
        assert_eq!(CommandType::C_PUSH, arthmethtic);
        assert_eq!("constant", arg1);
        assert_eq!(8, arg2);
        cmds.advance();
        let arthmethtic = cmds.command_type();
        let arg1 = cmds.arg1();
        let arg2 = cmds.arg2();
        assert_eq!(CommandType::C_PUSH, arthmethtic);
        assert_eq!("constant", arg1);
        assert_eq!(7, arg2);
        cmds.advance();
        let arthmethtic = cmds.command_type();
        let arg1 = cmds.arg1();
        assert_eq!(CommandType::C_ARITHMETIC, arthmethtic);
        assert_eq!("add", arg1);
    }
}
