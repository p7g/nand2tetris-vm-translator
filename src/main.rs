mod command;
mod lexer;
mod parser;
mod token;
mod translator;
mod assembly_builder;
mod code_gen;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use lexer::lex;
use parser::parse;
use translator::Translator;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let None = args.get(1) {
        panic!("Missing filename argument");
    }

    let path = Path::new(args.get(1).unwrap());

    let mut file = File::open(&path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let tokens = lex(&contents);

    let commands = parse(&tokens);

    let mut translator = Translator::new();
    translator.translate(commands);
    translator.write(&mut std::io::stdout())?;

    Ok(())
}
