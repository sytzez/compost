use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::tokenizer::tokenize;
use crate::runtime::evaluate::evaluate;
use crate::sem::semantic_analyser::analyse_ast;
use std::fs;

pub fn run_file(file_path: &str) -> String {
    let std = include_str!("resources/lib/std.compost");
    let code = fs::read_to_string(file_path).expect("Unable to read file");
    let all_code = std.to_string() + &code;

    run_or_show_error(&all_code)
}

pub fn run_code(code: &str) -> String {
    let std = include_str!("resources/lib/std.compost");
    let all_code = std.to_string() + &code;

    run_or_show_error(&all_code)
}

fn run_or_show_error(code: &str) -> String {
    match run(code) {
        Ok(result) => result,
        Err(error) => error.to_string(code),
    }
}

fn run(code: &str) -> CResult<String> {
    let mut tokens = tokenize(code)?;

    let ast = AbstractSyntaxTree::parse(&mut tokens)?;

    let context = analyse_ast(ast)?;

    let main_let = context.lets.resolve("Main", "")?;

    let result = evaluate(&main_let.borrow().evaluation, [].into(), None);

    let string = result.to_string(&context)?;

    Ok(string)
}
