use crate::ast::let_statement::{LetStatement, LetsStatement};
use crate::ast::module_statement::ModuleStatement;
use crate::ast::parser::Parser;
use crate::error::CResult;
use crate::lex::token::Token;
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// The abstract syntax tree of a whole program, containing all statements and expressions.
pub struct AbstractSyntaxTree {
    pub mods: Vec<ModuleStatement>,
    pub lets: Vec<LetStatement>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        Self {
            mods: vec![],
            lets: vec![],
        }
    }
}

impl Parser for AbstractSyntaxTree {
    fn matches(_tokens: &[LeveledToken]) -> bool {
        true
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let mut ast = AbstractSyntaxTree::new();

        while tokens.still_more() {
            if let Some(module) = ModuleStatement::maybe_parse(tokens)? {
                ast.mods.push(module)
            } else if let Some(mut lets) = LetsStatement::maybe_parse(tokens)? {
                ast.lets.append(&mut lets.lets)
            } else if matches!(tokens.token(), Token::Eof) {
                break;
            } else {
                return tokens.error(format!("Unexpected token {:?}", tokens.token()));
            }
        }

        Ok(ast)
    }
}
