use std::collections::HashMap;

struct Statement {
  state: char, // a or c
  value: Option<u32>,
  comp: Option<&'static str>,
  dest: Option<&'static str>,
  jump: Option<&'static str>,
}

impl Statement {
  pub fn new(
    state: char,
    value: Option<u32>,
    comp: Option<&'static str>,
    dest: Option<&'static str>,
    jump: Option<&'static str>,
  ) -> Self {
    Statement {
      state: state,
      value: value,
      comp: comp,
      dest: dest,
      jump: jump,
    }
  }

  fn code(&self) -> String {
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
    comp_map.insert("A".to_string(), "0110000");
    match comp {
      None => panic!("panic"),
      Some(x) => match comp_map.get(x) {
        None => panic!("invalid code, map missmached"),
        Some(x) => &x,
      },
    }
  }

  fn encode_dest(dest: Option<&str>) -> String {
    let mut result = 0;
    match dest {
      None => return format!("{:0>3b}", result),
      Some(x) => {
        if x.contains("M") {
          // bit for d3
          result += 1;
        }
        if x.contains("D") {
          // bit for d2
          result += 2;
        }
        if x.contains("A") {
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
        "JGT" => {
          return format!("{:0>3b}", 001);
        }
        _ => {
          todo!()
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
  }
}
