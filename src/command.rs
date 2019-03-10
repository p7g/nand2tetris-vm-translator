use super::token::Token;

#[derive(Debug)]
pub struct Command<'a> {
  pub name: &'a Token<'a>,
  args: Vec<&'a Token<'a>>,
}

impl<'a> Command<'a> {
  pub fn new(name: &'a Token) -> Command<'a> {
    Command {
      name,
      args: Vec::new(),
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
