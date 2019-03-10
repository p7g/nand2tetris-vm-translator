#[macro_use]
mod assembly_builder;
mod command;
mod lexer;
mod parser;
mod token;
mod translator;
mod code_gen;

use std::env;
use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use self::lexer::lex;
use self::parser::parse;
use self::translator::Translator;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let pathstr = match args.get(1) {
        Some(s) => s,
        None => return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Missing filename argument",
        )),
    };

    // FIXME: properly handle things, not unwrap
    let path = Path::new(pathstr);
    let dir = path.parent().unwrap();
    let filename = match path.file_stem() {
        Some(name) => name,
        None => unimplemented!(),
    };
    let output_file = Path::new(dir).join(Path::new(filename).with_extension("asm"));

    let meta = std::fs::metadata(pathstr)?;
    let paths: Vec<PathBuf> = if meta.is_file() {
        std::iter::once(PathBuf::from(pathstr)).collect()
    } else {
        path.read_dir()?.filter(|child| {
            let path = match child {
                Ok(c) => c.path(),
                Err(_) => return false,
            };
            path.is_file()
            && path.extension()
                .filter(|s| s.to_str().is_some()
                            && s.to_str().unwrap() == "vm")
                .is_some()
        }).filter_map(|dir| dir.ok().map(|d| d.path())).collect()
    };

    let mut translator = Translator::new();
    for path in &paths {
        let mut file = File::open(&path)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let tokens = lex(&contents);

        let commands = parse(&tokens);

        translator.translate_file(
            path.file_stem().unwrap().to_str().unwrap(),
            commands
        ).unwrap();
    }

    let mut output_file_stream = File::create(output_file)?;
    translator.write(&mut output_file_stream)?;

    Ok(())
}
