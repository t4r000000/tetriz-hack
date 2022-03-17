use std::io::{BufRead, BufReader, Read};
pub struct Commands {
    cmds: Vec<String>,
}

trait Parser {
    fn new<R: Read>(f: BufReader<R>) -> Self;
    // fn has_more_commands() -> bool;
    // fn advance() {}
    // fn command_type() -> String;
    // fn arg1() -> String;
    // fn arg2() -> i32;
}

impl Parser for Commands {
    fn new<R: Read>(mut f: BufReader<R>) -> Commands {
        let mut cmds: Vec<String> = Vec::new();
        let mut line = String::new();
        println!("loop 開始");
        loop {
            if f.read_line(&mut line).expect("panic! while readling Buf") == 0 {
                break;
            }
            cmds.push(line.to_string());
            line.clear();
        }
        Commands { cmds }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{Commands, Parser};
    use std::io::{BufReader, Cursor};

    #[test]
    fn test_ok() {
        let command = Cursor::new("push constant 7\npush constant 8");
        let readio = BufReader::new(command);
        let commands = Commands::new(readio);
        println!("print {:?}", commands.cmds);
        assert_eq!(2 + 2, 4);
    }
}
