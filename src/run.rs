use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::ast::parser::Parse;
use crate::error::CResult;
use crate::lex::tokenizer::tokenize;
use crate::runtime::evaluate::evaluate;
use crate::sem::semantic_analyser::analyse_ast;
use std::fs;

pub fn run_file(file_path: &str) -> String {
    let code = fs::read_to_string(file_path).expect("Unable to read file");

    run_code(&code)
}

pub fn run_code(code: &str) -> String {
    let std_code = include_str!("resources/lib/std.compost");
    let all_code = std_code.to_string() + code;

    match run(&all_code) {
        Ok(result) => result,
        Err(error) => error.to_string(std_code, &all_code),
    }
}

fn run(code: &str) -> CResult<String> {
    let mut tokens = tokenize(code)?;

    let ast = AbstractSyntaxTree::parse(&mut tokens)?;

    let context = analyse_ast(ast)?;

    let main_let = context.lets.resolve("Main", "")?;

    let result = evaluate(&main_let.borrow().evaluation, &[].into(), &None);

    let string = result.to_string(&context)?;

    Ok(string)
}
