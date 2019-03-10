#[derive(Debug)]
pub enum TokenType {
  Integer,
  Identifier,
  Newline,
}

#[derive(Debug)]
pub enum Value {
  None,
  Integer(i16),
}

#[derive(Debug)]
pub struct Token<'a> {
  pub type_: TokenType,
  pub lexeme: &'a str,
  pub line: usize,
  pub column: usize,
  pub value: Value,
}

impl<'a> Token<'a> {
  pub fn new(type_: TokenType, lexeme: &'a str, line: usize, column: usize) -> Token<'a> {
    Token {
      type_,
      lexeme,
      line,
      column,
      value: Value::None,
    }
  }

  pub fn with_value(mut self, value: Value) -> Token<'a> {
    self.value = value;
    self
  }
}
