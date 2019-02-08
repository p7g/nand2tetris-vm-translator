use super::token::Token;

#[derive(Debug)]
pub struct Command<'a> {
  pub name: &'a str,
  args: Vec<&'a Token<'a>>,
  pub line: usize,
  pub column: usize,
}

impl<'a> Command<'a> {
  pub fn new(name: &'a str, line: usize, column: usize) -> Command {
    Command {
      name,
      args: Vec::new(),
      line,
      column,
    }
  }

  pub fn push_arg(&mut self, arg: &'a Token<'a>) {
    self.args.push(arg);
  }

  pub fn num_args(&self) -> usize {
    self.args.len()
  }

  pub fn arg(&self, i: usize) -> &Token<'a> {
    self.args[i]
  }
}
