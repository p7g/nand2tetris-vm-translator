use std::io::{Result, Write};

use super::assembly_builder::AssemblyBuilder;
use super::command::Command;
use super::token::Token;

#[derive(Debug)]
pub struct Translator {
  assembly: AssemblyBuilder,
}

impl Translator {
  pub fn new() -> Translator {
    Translator {
      assembly: AssemblyBuilder::new(),
    }
  }

  pub fn translate(&mut self, commands: Vec<Command>) {
    for command in commands {
      match command.name {
        "push" | "pop" => {
          if command.num_args() != 2 {
            panic!(
              "Expected 2 arguments for {} at line {}, column {}",
              command.name, command.line, command.column
            );
          }
          if let Token::Identifier { name, line, column } = command.arg(0) {
            if !is_valid_segment(name) {
              panic!(
                "Unknown segment '{}' at line {}, column {}",
                name, line, column
              );
            }
            if let Token::Integer {
              value,
              line,
              column,
            } = command.arg(1)
            {

            } else {
              panic!(
                "Expected second argument to be integer at line {}, column {}",
                command.line, command.column
              );
            }
          } else {
            panic!(
              "Expected first argument to be identifier at line {}, column {}",
              command.line, command.column
            );
          }
        }
        _ => panic!(
          "Unknown command '{}' at line {}, column {}",
          command.name, command.line, command.column
        ),
      };
    }
  }

  pub fn write(&self, stream: &mut Write) -> Result<()> {
    self.assembly.write(stream)
  }
}

fn is_valid_segment(name: &str) -> bool {
  false
}
