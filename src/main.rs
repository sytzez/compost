use std::{env, fs};
use std::borrow::Borrow;
use crate::instance::Instance;
use crate::parser::parse_tokens;
use crate::raw_value::RawValue;
use crate::scope::path;
use crate::tokenizer::tokenize;

mod token;
mod scope;
mod class;
mod std_lib;
mod module;
mod expression;
mod strukt;
mod definition;
mod typ;
mod raw_value;
mod instance;
mod lett;
mod trayt;
mod tokenizer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(file_path) = args.get(1) {
        let std = fs::read_to_string("lib/std.compost")
            .expect("Unable to read lib/std.compost");

        let code = fs::read_to_string(file_path)
            .expect("Unable to read file");

        let all_code = std + &code;

        let result = run_code(&all_code);

        println!("{}", result);
    } else {
        println!("No file path given")
    }
}

pub fn run_code(code: &str) -> String {
    let scope = parse_tokens(&tokenize(code));

    let main = scope.lett(&path("Main"));

    let resolved_instance = main.resolve([].into(), &scope);

    let raw_value = match resolved_instance.borrow() {
        Instance::Raw(raw_value) => raw_value,
        Instance::Struct(strukt) => strukt.value("value"),
        _ => return "Resolved value of main can not be represented as a string".into(),
    };

    match raw_value {
        RawValue::String(value) => value.clone(),
        RawValue::Int(value) => value.to_string(),
        RawValue::UInt(value) => value.to_string(),
    }
}
