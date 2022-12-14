use crate::ast::class_statement::ClassStatement;
use crate::ast::def_statement::{DefStatement, DefsStatement};
use crate::ast::let_statement::{LetStatement, LetsStatement};
use crate::ast::parser::{parse_global, Parse};
use crate::ast::struct_statement::StructStatement;
use crate::ast::trait_statement::{TraitStatement, TraitsStatement};
use crate::ast::using_statement::{SingleUsingStatement, UsingStatement};
use crate::ast::Statement;
use crate::error::{CResult, ErrorMessage};
use crate::lex::token::{Kw, Token};
use std::ops::Range;

use crate::lex::tokens::Tokens;

/// A whole module.
pub struct ModuleStatement {
    pub name: String,
    pub class: Option<ClassStatement>,
    pub strukt: Option<StructStatement>,
    pub traits: Vec<TraitStatement>,
    pub defs: Vec<DefStatement>,
    pub lets: Vec<LetStatement>,
    pub using: Vec<SingleUsingStatement>,
    token_range: Range<usize>,
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
            using: vec![],
            token_range: Range::default(),
        }
    }
}

impl Parse for ModuleStatement {
    fn matches(tokens: &Tokens) -> bool {
        matches!(tokens.token(), Token::Kw(Kw::Mod))
    }

    fn parse(tokens: &mut Tokens) -> CResult<Self> {
        let base_level = tokens.level();
        let token_start = tokens.position();
        tokens.step();

        let name = parse_global(tokens)?;

        let mut statement = ModuleStatement::new(name);

        while tokens.deeper_than(base_level) {
            tokens.expect("class, struct, traits, defs, lets or using");

            if let Some(class) = ClassStatement::maybe_parse(tokens)? {
                if statement.class.is_some() {
                    return class.error(ErrorMessage::DuplicateClass(statement.name));
                }
                if statement.strukt.is_some() {
                    return class.error(ErrorMessage::ClassAndStruct(statement.name));
                }
                statement.class = Some(class);
            } else if let Some(strukt) = StructStatement::maybe_parse(tokens)? {
                if statement.strukt.is_some() {
                    return strukt.error(ErrorMessage::DuplicateStruct(statement.name));
                }
                if statement.class.is_some() {
                    return strukt.error(ErrorMessage::ClassAndStruct(statement.name));
                }
                statement.strukt = Some(strukt);
            } else if let Some(mut traits) = TraitsStatement::maybe_parse(tokens)? {
                statement.traits.append(&mut traits.traits);
            } else if let Some(mut defs) = DefsStatement::maybe_parse(tokens)? {
                statement.defs.append(&mut defs.defs);
            } else if let Some(mut lets) = LetsStatement::maybe_parse(tokens)? {
                statement.lets.append(&mut lets.lets);
            } else if let Some(mut using) = UsingStatement::maybe_parse(tokens)? {
                statement.using.append(&mut using.lines);
            } else {
                return tokens.unexpected_token_error();
            }
        }

        statement.token_range = token_start..tokens.position();

        Ok(statement)
    }
}

impl Statement for ModuleStatement {
    fn token_range(&self) -> &Range<usize> {
        &self.token_range
    }
}
