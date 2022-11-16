use std::fs;
use crate::parser::parse_tokens;
use crate::scope::path;
use crate::tokenizer::tokenize;

pub fn run_file(file_path: &str) -> String {
    let std = fs::read_to_string("lib/std.compost").expect("Unable to read lib/std.compost");

    let code = fs::read_to_string(file_path).expect("Unable to read file");

    let all_code = std + &code;

    run_code(&all_code)
}

pub fn run_code(code: &str) -> String {
    let scope = parse_tokens(&tokenize(code));

    scope
        .lett(&path("Main"))
        .resolve([].into(), &scope)
        .to_string(&scope)
}
