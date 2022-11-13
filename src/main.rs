use crate::instance::Instance;
use crate::parser::parse_tokens;
use crate::raw_value::RawValue;
use crate::scope::path;
use crate::tokenizer::tokenize;
use std::{env, fs};

mod class;
mod definition;
mod expression;
mod instance;
mod lett;
mod module;
mod parser;
mod raw_value;
mod scope;
mod strukt;
mod token;
mod tokenizer;
mod trayt;
mod typ;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let std = fs::read_to_string("lib/std.compost").expect("Unable to read lib/std.compost");

        let code = fs::read_to_string(file_path).expect("Unable to read file");

        let all_code = std + &code;

        let result = run_code(&all_code);

        println!("{}", result);
    } else {
        println!("Specify a source file to run")
    }
}

pub fn run_code(code: &str) -> String {
    let scope = parse_tokens(&tokenize(code));

    scope
        .lett(&path("Main"))
        .resolve([].into(), &scope)
        .to_string(&scope)
}
