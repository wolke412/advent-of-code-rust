pub struct Pair {
  left: u8,
  right: u8
}

impl Pair {

  pub fn new( l: char, r: char  ) -> Pair {
    Pair {
      left:  l as u8 - b'A' + 1,
      right: r as u8 - b'X' + 1
    }
  }

  fn won ( &self ) -> bool {
    self.right > self.left
  }

  fn drawn ( &self ) -> bool {
    self.right == self.left
  }

  pub fn result ( &self ) -> u8 {

    println!("Pair: {} - {}", self.left, self.right);
    
    if self.won() {
      return self.right + 6;
    }

    if self.drawn() {
      return self.right + 3;
    }

    self.right
  }
}