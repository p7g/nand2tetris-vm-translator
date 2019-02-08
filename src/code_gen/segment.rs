#[derive(Debug)]
pub enum Segment {
  Constant,
}

impl std::fmt::Display for Segment {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", match self {
      Segment::Constant => "constant",
    })
  }
}
