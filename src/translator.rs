use std::io::{Result, Write};

use super::assembly_builder::AssemblyBuilder;
use super::command::Command;
use super::token::{TokenType, Value};
use super::code_gen::segment::Segment;

#[derive(Debug)]
pub struct Translator {
  assembly: AssemblyBuilder,
  current_function_name: Option<String>,
}

impl Translator {
  pub fn new() -> Translator {
    Translator {
      assembly: AssemblyBuilder::new(),
      current_function_name: None,
    }
  }

  pub fn translate_file(&mut self, filename: &str, commands: Vec<Command>) -> std::result::Result<(), String> {
    for command in commands {

      macro_rules! err {
        ( $message:expr ) => {
          Err(new_error(String::from($message), filename, &command))
        };
        ( $fmtstr:expr, $($fmtargs:expr),* ) => {
          Err(new_error(
            format!($fmtstr, $($fmtargs),*),
            filename,
            &command,
          ))
        };
      }

      match command.name.lexeme { // FIXME: should have CommandType enum and match on that (or have command be enum)
        "push" | "pop" => {
          if command.num_args() != 2 {
            return err!("Expected 2 arguments for {}", command.name.lexeme);
          }
          let first_arg = command.arg(0);
          let second_arg = command.arg(1);
          if let TokenType::Identifier = first_arg.type_ {
            if !Segment::is_valid_name(first_arg.lexeme) {
              return err!("Unknown segment '{}'", first_arg.lexeme);
            }
            if let Value::Integer(index) = second_arg.value {
              let segment = Segment::from_name(first_arg.lexeme, filename)?;

              if command.name.lexeme == "pop" {
                if !segment.is_writable() {
                  return err!("Can't pop into read-only segment {}", segment);
                }
                pop!(self.assembly, segment, index);
              }
              else {
                if let Segment::Constant = segment {
                  push_constant!(self.assembly, index);
                } else {
                  push!(self.assembly, segment, index);
                }
              }
            } else {
              return err!("Expected second argument to be integer");
            }
          } else {
            return err!("Expected first argument to be identifier");
          }
        }

        "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
          if command.num_args() > 0 {
            return err!("Expected no arguments for command {}", command.name.lexeme);
          }

          match command.name.lexeme {
            "add" => add!(self.assembly),
            "sub" => sub!(self.assembly),
            "neg" => neg!(self.assembly),
            "eq" => eq!(self.assembly),
            "gt" => gt!(self.assembly),
            "lt" => lt!(self.assembly),
            "and" => and!(self.assembly),
            "or" => or!(self.assembly),
            "not" => not!(self.assembly),
            _ => unreachable!(),
          };
        },

        "label" | "goto" | "if-goto" => {
          if command.num_args() != 1 {
            return err!("Expected 1 argument for {}",
              command.name.lexeme);
          }
          let first_arg = command.arg(0);

          if let TokenType::Identifier = first_arg.type_ {
            let fn_name = match &self.current_function_name {
              None => return err!("Cannot use {} in non-function context", command.name.lexeme),
              Some(name) => name,
            };
            match command.name.lexeme {
              "label" => label!(self.assembly, fn_name, first_arg.lexeme),
              "goto" => goto!(self.assembly, fn_name, first_arg.lexeme),
              "if-goto" => if_goto!(self.assembly, fn_name, first_arg.lexeme),
              _ => unreachable!(),
            };
          } else {
            return err!("Expected argument to {} to be identifier", command.name.lexeme);
          }
        },

        "function" | "call" => {
          if command.num_args() != 2 {
            return err!("Expected 2 arguments for {}", command.name.lexeme);
          }
          let first_arg = command.arg(0);
          let second_arg = command.arg(1);

          let function_name: &str;
          if let TokenType::Identifier = first_arg.type_ {
            function_name = first_arg.lexeme;
          } else {
            return err!("Expected first argument of {} to be identifier", command.name.lexeme);
          }

          let num_vars: i16;
          if let Value::Integer(index) = second_arg.value {
            num_vars = index;
          } else {
            return err!("Expected second argument of {} to be integer", command.name.lexeme);
          }

          match command.name.lexeme {
            "function" => {
              self.current_function_name = Some(String::from(function_name));
              function!(self.assembly, function_name, num_vars);
            },
            "call" => call!(self.assembly, function_name, num_vars),
            _ => unreachable!(),
          };
        },

        "return" => {
          return_!(self.assembly);
        },

        _ => {
          return err!("Unknown command '{}'", command.name.lexeme);
        },
      }
    };

    Ok(())
  }

  pub fn write(&self, stream: &mut Write) -> Result<()> {
    self.assembly.write(stream)
  }
}

fn new_error(message: String, filename: &str, command: &Command) -> String {
  format!(
    "{} at {}.vm line {}, column {}",
    message, filename, command.name.line, command.name.column,
  )
}
