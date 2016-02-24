pub trait A {
  type T: B + C;

  fn a_work(&mut self) -> u32 {
    1
  }

  fn a_req_b_and_c(&mut self) -> u32 {
    self.a_work() + self.as_b().b_work() + self.as_c().c_work()
  }

  fn as_b(&mut self) -> &mut Self::T;
  fn as_c(&mut self) -> &mut Self::T;
}


pub trait B {
  type T: A;

  fn b_work(&mut self) -> u32 {
    2
  }

  fn b_req_a(&mut self) -> u32 {
    self.b_work() + self.as_a().a_work()
  }

  fn as_a(&mut self) -> &mut Self::T;
}


pub trait C {
  type T: A;

  fn c_work(&mut self) -> u32 {
    3
  }

  fn c_req_a(&mut self) -> u32 {
    self.c_work() + self.as_a().a_work()
  }

  fn as_a(&mut self) -> &mut Self::T;
}


#[cfg(test)]
mod test {
  use super::*;

  struct Abc;

  impl A for Abc {
    type T = Abc;
    fn as_b(&mut self) -> &mut Abc { self }
    fn as_c(&mut self) -> &mut Abc { self }
  }

  impl B for Abc {
    type T = Abc;
    fn as_a(&mut self) -> &mut Abc { self }
  }

  impl C for Abc {
    type T = Abc;
    fn as_a(&mut self) -> &mut Abc { self }
  }

  #[test]
  fn it_works() {
    assert_eq!(6, Abc.a_req_b_and_c());
    assert_eq!(3, Abc.b_req_a());
    assert_eq!(4, Abc.c_req_a());
  }
}
