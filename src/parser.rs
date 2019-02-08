use super::token::Token;
use super::command::{Command};

pub fn parse<'a>(tokens: &'a Vec<Token<'a>>) -> Vec<Command<'a>> {
  let mut commands = Vec::new();

  let mut command: Option<Command> = None;
  for token in tokens.iter() {
    match token {
      Token::Newline => {
        if let Some(c) = command {
          commands.push(c);
          command = None;
        }
      },
      ident @ Token::Identifier {..} => {
        if let Some(mut c) = command {
          c.push_arg(ident);
          command = Some(c);
        }
        else {
          if let Token::Identifier {name, line, column} = ident {
            command = Some(Command::new(name, *line, *column));
          }
        }
      },
      int @ Token::Integer {..} => {
        if let Some(mut c) = command {
          c.push_arg(int);
          command = Some(c);
        }
        else {
          if let Token::Integer {line, column, ..} = int {
            panic!("Unexpected integer at line {}, column {}", line, column);
          }
        }
      },
    }
  }

  commands
}
