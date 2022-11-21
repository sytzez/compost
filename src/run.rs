use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::tokenizer::tokenize;
use crate::sem::semantic_analyser::analyse_ast;
use std::fs;

pub fn run_file(file_path: &str) -> CResult<String> {
    let std = fs::read_to_string("lib/std.compost").expect("Unable to read lib/std.compost");

    let code = fs::read_to_string(file_path).expect("Unable to read file");

    let all_code = std + &code;

    run_code(&all_code)
}

pub fn run_code(code: &str) -> CResult<String> {
    let mut tokens = tokenize(code)?;

    let ast = AbstractSyntaxTree::parse(&mut tokens)?;

    let context = analyse_ast(ast)?;

    let _main_let = context.lets.resolve("Main", "")?;

    // let result = main_let.into_inner().unwrap().resolve([].into(), &scope).to_string(&scope);
    let result = "Test".to_string();

    Ok(result)
}
