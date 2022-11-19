use crate::ast::class_statement::ClassStatement;
use crate::ast::def_statement::{DefsStatement, DefStatement};
use crate::ast::let_statement::{LetsStatement, LetStatement};
use crate::ast::parser::{parse_global, Parser};
use crate::ast::struct_statement::StructStatement;
use crate::ast::trait_statement::{TraitsStatement, TraitStatement};
use crate::error::CResult;
use crate::lex::token::{Kw, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::lex::tokens::Tokens;

/// A whole module.
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

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        tokens.step();

        let name = parse_global(tokens)?;
        tokens.step();

        let mut statement = ModuleStatement::new(name);

        while tokens.deeper_than(base_level) {
            if let Some(class) = ClassStatement::maybe_parse(tokens)? {
                if statement.class.is_some() {
                    return tokens.error("Can't define more than one class per module".to_string())
                }

                if statement.strukt.is_some() {
                    return tokens.error("Can't define a class for a module that already has a struct".to_string())
                }

                statement.class = Some(class);
            } else if let Some(strukt) = StructStatement::maybe_parse(tokens)? {
                if statement.strukt.is_some() {
                    return tokens.error("Can't define more than one struct per module".to_string())
                }

                if statement.class.is_some() {
                    return tokens.error("Can't define a struct for a module that already has a class".to_string())
                }

                statement.strukt = Some(strukt);
            } else if let Some(mut traits) = TraitsStatement::maybe_parse(tokens)? {
                statement.traits.append(&mut traits.traits);
            } else if let Some(mut defs) = DefsStatement::maybe_parse(tokens)? {
                statement.defs.append(&mut defs.defs);
            } else if let Some(mut lets) = LetsStatement::maybe_parse(tokens)? {
                statement.lets.append(&mut lets.lets);
            } else {
                return tokens.error(format!("Unexpected token {:?}", tokens.token()))
            }
        }

        Ok(statement)
    }
}

