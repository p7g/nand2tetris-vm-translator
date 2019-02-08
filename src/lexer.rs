use super::token::{Token};

#[derive(Debug)]
enum State {
  None,

  InIdentifier,
  InNumber,
}

pub fn lex<'a>(source: &'a String) -> Vec<Token<'a>> {
  let mut tokens: Vec<Token<'a>> = Vec::new();
  let mut line = 1;
  let mut column = 0;

  let mut state = State::None;
  let mut lexeme = String::new();
  let mut chars = source.char_indices().peekable();

  let mut start: usize = 0;
  let mut current_line: usize = 0;
  let mut current_column: usize = 0;
  let mut position: usize = 0;
  let mut current: char = '\0';
  let mut do_column = true;

  loop {
    let (new_pos, next) = chars.peek().unwrap_or(&(0, '\0'));

    state = match state {
      State::None => {
        start = *new_pos;
        current_line = line;
        current_column = column + 1;
        lexeme = String::new();
        match next {
          '\r' | ' ' | '\t' => State::None,
          '\n' => {
            line += 1;
            column = 0;
            do_column = false;
            tokens.push(Token::Newline);
            State::None
          }
          '0'...'9' => State::InNumber,
          'a'...'z' | 'A'...'Z' | '_' => State::InIdentifier,
          '\0' => break,
          _ => panic!("what is this? {} at {}:{}", next, line, column),
        }
      }
      State::InIdentifier => {
        lexeme.push(current);
        match next {
          'a'...'z' | 'A'...'Z' | '_' | '0'...'9' => State::InIdentifier,
          _ => {
            tokens.push(Token::Identifier {
              name: source.get(start..=position).unwrap(),
              line: current_line,
              column: current_column,
            });
            if *next == '\n' {
              line += 1;
              column = 0;
              tokens.push(Token::Newline);
              do_column = false;
            }
            State::None
          }
        }
      }
      State::InNumber => {
        lexeme.push(current);
        match next {
          '0'...'9' => State::InNumber,
          _ => {
            tokens.push(Token::Integer {
              value: i64::from_str_radix(&lexeme, 10).unwrap(),
              line: current_line,
              column: current_column,
            });
            if *next == '\n' {
              line += 1;
              column = 0;
              tokens.push(Token::Newline);
              do_column = false;
            }
            State::None
          }
        }
      },
    };

    if do_column {
      column += 1;
    }
    do_column = true;
    if let Some((new_pos, next)) = chars.next() {
      position = new_pos;
      current = next;
    } else {
      break;
    }
  }

  tokens
}
