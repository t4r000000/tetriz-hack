use std::collections::HashMap;

struct Statement {
  state: char, // a or c
  value: Option<u32>,
  comp: Option<String>,
  dest: Option<String>,
  jump: Option<String>,
}

impl Statement {
  pub fn new(
    state: char,
    value: Option<u32>,
    comp: Option<String>,
    dest: Option<String>,
    jump: Option<String>,
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
          Statement::comp_map(&self.comp),
          Statement::decode_dest(&self.dest),
          Statement::decode_jump(&self.jump)
        )
      }
      _ => todo!(),
    }
  }

  fn comp_map(comp: &Option<String>) -> String {
    let mut comp_map = HashMap::new();
    comp_map.insert("A".to_string(), "0110000".to_string());
    match comp {
      None => panic!("panic"),
      Some(x) => comp_map.get(x).unwrap().to_string(),
    }
  }

  fn decode_dest(dest: &Option<String>) -> String {
    match dest {
      None => "000".to_string(),
      Some(x) => todo!(),
    }
  }

  fn decode_jump(jump: &Option<String>) -> String {
    match jump {
      None => "000".to_string(),
      Some(x) => todo!(),
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
    let state_c = Statement::new('C', None, Some("A".to_string()), None, None);
    assert_eq!(state_c.code(), "1110110000000000");
  }
}
