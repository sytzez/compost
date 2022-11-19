use std::ops::AddAssign;
use crate::ast::class_statement::ClassStatement;
use crate::ast::def_statement::{DefsStatement, DefStatement};
use crate::ast::let_statement::{LetsStatement, LetStatement};
use crate::ast::parser::Parser;
use crate::ast::struct_statement::StructStatement;
use crate::ast::trait_statement::{TraitsStatement, TraitStatement};
use crate::error::{CResult, error};
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::parser::parse_global;

pub struct ModuleStatement {
    pub name: String,
    pub class: Option<ClassStatement>,
    pub strukt: Option<StructStatement>,
    pub traits: Vec<TraitStatement>,
    pub defs: Vec<DefStatement>,
    pub lets: Vec<LetStatement>,
}

impl ModuleStatement {
    fn new(name: String) -> Self {
        Self {
            name,
            class: None,
            strukt: None,
            traits: vec![],
            defs: vec![],
            lets: vec![],
        }
    }
}

impl Parser for ModuleStatement {
    fn matches(tokens: &[LeveledToken]) -> bool {
        matches!(tokens[0].0, Token::Kw(Kw::Mod))
    }

    fn parse(tokens: &[LeveledToken], position: &mut usize) -> CResult<Self> {
        let base_level = tokens[*position].1;
        position.add_assign(1);

        let name = parse_global(&tokens[*position], base_level)?;
        position.add_assign(1);

        let mut statement = ModuleStatement::new(name);

        while *position < tokens.len() {
            if tokens[*position].1 <= base_level {
                break;
            }

            if let Some(class) = ClassStatement::parse_maybe(tokens, position)? {
                if statement.class.is_some() {
                    return error("Can't define more than one class per module".to_string())
                }

                if statement.strukt.is_some() {
                    return error("Can't define a class for a module that already has a struct".to_string())
                }

                statement.class = Some(class);
            } else if let Some(strukt) = StructStatement::parse_maybe(tokens, position)? {
                if statement.strukt.is_some() {
                    return error("Can't define more than one struct per module".to_string())
                }

                if statement.class.is_some() {
                    return error("Can't define a struct for a module that already has a class".to_string())
                }

                statement.strukt = Some(strukt);
            } else if let Some(mut traits) = TraitsStatement::parse_maybe(tokens, position)? {
                statement.traits.append(&mut traits.traits);
            } else if let Some(mut defs) = DefsStatement::parse_maybe(tokens, position)? {
                statement.defs.append(&mut defs.defs);
            } else if let Some(mut lets) = LetsStatement::parse_maybe(tokens, position)? {
                statement.lets.append(&mut lets.lets);
            }
        }

        Ok(statement)
    }
}

