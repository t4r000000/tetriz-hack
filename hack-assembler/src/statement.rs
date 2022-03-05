use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Statement<'a> {
    state: char, // a or c
    value: Option<u32>,
    comp: Option<&'a str>,
    dest: Option<&'a str>,
    jump: Option<&'a str>,
}

impl<'a> Statement<'a> {
    pub fn new(
        state: char,
        value: Option<u32>,
        comp: Option<&'a str>,
        dest: Option<&'a str>,
        jump: Option<&'a str>,
    ) -> Self {
        Statement {
            state,
            value,
            comp,
            dest,
            jump,
        }
    }

    pub fn code(&self) -> String {
        match self.state {
            'A' => format!("0{:0>15b}", self.value.unwrap()),
            'C' => {
                format!(
                    "111{}{}{}",
                    Statement::comp_map(self.comp),
                    Statement::encode_dest(self.dest),
                    Statement::encode_jump(self.jump)
                )
            }
            _ => todo!(),
        }
    }

    fn comp_map(comp: Option<&str>) -> &str {
        let mut comp_map = HashMap::new();
        comp_map.insert("0", "0101010");
        comp_map.insert("1", "0111111");
        comp_map.insert("-1", "0111010");
        comp_map.insert("D", "0001100");
        comp_map.insert("A", "0110000");
        comp_map.insert("!D", "0001101");
        comp_map.insert("!A", "0110001");
        comp_map.insert("-D", "0001111");
        comp_map.insert("-A", "0110011");
        comp_map.insert("D+1", "0011111");
        comp_map.insert("A+1", "1101111");
        comp_map.insert("D-1", "0001110");
        comp_map.insert("A-1", "0110010");
        comp_map.insert("D+A", "0000010");
        comp_map.insert("D-A", "0010011");
        comp_map.insert("A-D", "0000111");
        comp_map.insert("D&A", "0000000");
        comp_map.insert("D|A", "0010101");

        comp_map.insert("M", "1110000");
        comp_map.insert("!M", "1110001");
        comp_map.insert("-M", "1110011");
        comp_map.insert("M+1", "1110111");
        comp_map.insert("M-1", "1110010");
        comp_map.insert("D+M", "1000010");
        comp_map.insert("D-M", "1010011");
        comp_map.insert("M-D", "1000111");
        comp_map.insert("D&M", "1000000");
        comp_map.insert("D|M", "1010101");

        match comp {
            None => panic!("panic"),
            Some(x) => match comp_map.get(x) {
                None => panic!("invalid code, map missmached{:?}", comp),
                Some(x) => x,
            },
        }
    }

    fn encode_dest(dest: Option<&str>) -> String {
        let mut result = 0;
        match dest {
            None => return format!("{:0>3b}", result),
            Some(x) => {
                if x.contains('M') {
                    // bit for d3
                    result += 1;
                }
                if x.contains('D') {
                    // bit for d2
                    result += 2;
                }
                if x.contains('A') {
                    // bit for d1
                    result += 4;
                }
            }
        }
        return format!("{:0>3b}", result);
    }

    fn encode_jump(jump: Option<&str>) -> String {
        match jump {
            None => "000".to_string(),
            Some(x) => match x {
                "JGT" => "001".to_string(),
                "JEQ" => "010".to_string(),
                "JGE" => "011".to_string(),
                "JLT" => "100".to_string(),
                "JNE" => "101".to_string(),
                "JLE" => "110".to_string(),
                "JMP" => "111".to_string(),
                _ => {
                    panic!("{} is invalid Jump Code", x)
                }
            },
        }
    }
}

#[cfg(test)]
mod statement_tests {
    use super::*;

    #[test]
    fn test_ok() {
        let state_a = Statement::new('A', Some(1), None, None, None);
        assert_eq!(state_a.code(), "0000000000000001");
        let state_a = Statement::new('A', Some(32767), None, None, None);
        assert_eq!(state_a.code(), "0111111111111111");
        let state_c = Statement::new('C', None, Some("A"), None, None);
        assert_eq!(state_c.code(), "1110110000000000");
        let state_c = Statement::new('C', None, Some("A"), Some("M"), None);
        assert_eq!(state_c.code(), "1110110000001000");
        let state_c = Statement::new('C', None, Some("A"), Some("MD"), None);
        assert_eq!(state_c.code(), "1110110000011000");
        let state_c = Statement::new('C', None, Some("A"), Some("AMD"), None);
        assert_eq!(state_c.code(), "1110110000111000");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JGT"));
        assert_eq!(state_c.code(), "1110110000110001");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JEQ"));
        assert_eq!(state_c.code(), "1110110000110010");
        let state_c = Statement::new('C', None, Some("A"), Some("AM"), Some("JGE"));
        assert_eq!(state_c.code(), "1110110000101011");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JLT"));
        assert_eq!(state_c.code(), "1110110000110100");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JNE"));
        assert_eq!(state_c.code(), "1110110000110101");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JLE"));
        assert_eq!(state_c.code(), "1110110000110110");
        let state_c = Statement::new('C', None, Some("A"), Some("AD"), Some("JMP"));
        assert_eq!(state_c.code(), "1110110000110111");
    }
}
