use super::token::{Token, TokenType};
use super::command::{Command};

pub fn parse<'a>(tokens: &'a Vec<Token<'a>>) -> Vec<Command<'a>> {
  let mut commands = Vec::new();

  let mut command: Option<Command> = None;
  for token in tokens.iter() {
    match token.type_ {
      TokenType::Newline => {
        if let Some(c) = command {
          commands.push(c);
          command = None;
        }
      },
      TokenType::Identifier => {
        if let Some(mut c) = command {
          c.push_arg(token);
          command = Some(c);
        }
        else {
          command = Some(Command::new(token));
        }
      },
      TokenType::Integer => {
        if let Some(mut c) = command {
          c.push_arg(token);
          command = Some(c);
        }
        else {
          panic!("Unexpected integer at line {}, column {}", token.line, token.column);
        }
      },
    }
  }

  commands
}
