#[derive(Debug)]
pub enum Token<'a> {
  Integer {
    value: i64,
    line: usize,
    column: usize,
  },
  Identifier {
    name: &'a str,
    line: usize,
    column: usize,
  },
  Newline,
}
