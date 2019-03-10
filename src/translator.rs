use std::io::{Result, Write};

use super::assembly_builder::AssemblyBuilder;
use super::command::Command;
use super::token::{TokenType, Value};
use super::code_gen::segment::Segment;

#[derive(Debug)]
pub struct Translator<'a> {
  assembly: AssemblyBuilder,
  function_stack: Vec<&'a str>,
}

impl<'a> Translator<'a> {
  pub fn new() -> Translator<'a> {
    Translator {
      assembly: AssemblyBuilder::new(),
      function_stack: vec![],
    }
  }

  pub fn translate_file(&mut self, filename: &str, commands: Vec<Command<'a>>) -> std::result::Result<(), String> {
    for command in commands {
      match command.name.lexeme { // FIXME: should have CommandType enum and match on that (or have command be enum)
        "push" | "pop" => {
          if command.num_args() != 2 {
            return Err(format!(
              "Expected 2 arguments for {} at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }
          let first_arg = command.arg(0);
          let second_arg = command.arg(1);
          if let TokenType::Identifier = first_arg.type_ {
            if !Segment::is_valid_name(first_arg.lexeme) {
              return Err(format!(
                "Unknown segment '{}' at line {}, column {}",
                first_arg.lexeme, first_arg.line, first_arg.column,
              ));
            }
            if let Value::Integer(index) = second_arg.value {
              let segment = Segment::from_name(first_arg.lexeme, filename)?;

              if command.name.lexeme == "pop" {
                if !segment.is_writable() {
                  return Err(format!("Can't pop into read-only segment {}", segment));
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
              return Err(format!(
                "Expected second argument to be integer at line {}, column {}",
                command.name.line, command.name.column
              ));
            }
          } else {
            return Err(format!(
              "Expected first argument to be identifier at line {}, column {}",
              command.name.line, command.name.column
            ));
          }
        }

        "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
          if command.num_args() > 0 {
            return Err(format!("Expected no arguments for command {}", command.name.lexeme));
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
            return Err(format!(
              "Expected 1 argument for {} at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }
          let first_arg = command.arg(0);

          if let TokenType::Identifier = first_arg.type_ {
            let fn_name = match self.function_stack.last() {
              None => return Err(format!(
                "Cannot use {} in non-function context at line {}, column {}",
                command.name.lexeme, command.name.line, command.name.column,
              )),
              Some(name) => name,
            };
            match command.name.lexeme {
              "label" => label!(self.assembly, fn_name, first_arg.lexeme),
              "goto" => goto!(self.assembly, fn_name, first_arg.lexeme),
              "if-goto" => if_goto!(self.assembly, fn_name, first_arg.lexeme),
              _ => unreachable!(),
            };
          } else {
            return Err(format!(
              "Expected argument to {} to be identifier at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }
        },

        "function" | "call" => {
          if command.num_args() != 2 {
            return Err(format!(
              "Expected 2 arguments for {} at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }
          let first_arg = command.arg(0);
          let second_arg = command.arg(1);

          let function_name: &'a str;
          if let TokenType::Identifier = first_arg.type_ {
            function_name = first_arg.lexeme;
          } else {
            return Err(format!(
              "Expected first argument of {} to be identifier at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }

          let num_vars: i16;
          if let Value::Integer(index) = second_arg.value {
            num_vars = index;
          } else {
            return Err(format!(
              "Expected second argument of {} to be integer at line {}, column {}",
              command.name.lexeme, command.name.line, command.name.column,
            ));
          }

          match command.name.lexeme {
            "function" => {
              self.function_stack.push(function_name);
              function!(self.assembly, function_name, num_vars);
            },
            "call" => call!(self.assembly, function_name, num_vars),
            _ => unreachable!(),
          };
        },

        "return" => {

        },

        _ => {
          return Err(format!(
            "Unknown command '{}' at line {}, column {}",
            command.name.lexeme, command.name.line, command.name.column
          ))
        },
      }
    };

    Ok(())
  }

  pub fn write(&self, stream: &mut Write) -> Result<()> {
    self.assembly.write(stream)
  }
}
