struct Statement {
  state: char, // a or c
  value: Option<u16>,
  comp: Option<String>,
  dest: Option<String>,
  jump: Option<String>,
}

impl Statement {
  pub fn new(
    state: char,
    value: Option<u16>,
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
    if self.state == 'A' {
      String::from("0000000000000001")
    } else {
      todo!()
    }
  }
}

#[cfg(test)]
mod statement_tests {
  use super::*;

  #[test]
  fn test_ok() {
    let stateA1 = Statement::new('A', Some(1), None, None, None);
    assert_eq!(stateA1.code(), "0000000000000001")
  }
}
