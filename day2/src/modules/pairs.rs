pub struct Pair {
  left: u8,
  right: u8
}

impl Pair {

  pub fn new( l: char, r: char  ) -> Pair {
    Pair {
      left:  get_value(l),
      right: get_value(r)
    }
  }

  fn won ( &self ) -> bool {
    self.right > self.left
  }

  fn drawn ( &self ) -> bool {
    self.right == self.left
  }

  pub fn result ( &self ) -> u8 {

    print!("{} - {}", self.left, self.right);
    
    if self.won() {
      return self.right + 6;
    }

    if self.drawn() {
      return self.right + 3;
    }

    self.right
  }
}

pub fn get_value ( letter: char  ) -> u8 {

  match letter {
    'X' | 'A' => 1,
    'Y' | 'B' => 2,
    'Z' | 'C' => 3,

    _ => 0
  }

}