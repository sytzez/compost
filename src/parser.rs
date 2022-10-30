use crate::class::Class;
use crate::definition::Definition;
use crate::lett::Let;
use crate::module::Module;
use crate::scope::{path, Scope};
use crate::strukt::Struct;
use crate::token::{Kw, Token};
use crate::tokenizer::LeveledToken;
use crate::trayt::Trait;

// Parse a series of leveled tokens into a scope.
pub fn parse_tokens(tokens: &[LeveledToken]) -> Scope {
    let mut position = 0;

    let mut scope = Scope::new();

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 != 0 {
            panic!("Unexpected level")
        }

        match leveled_token.0 {
            // TODO: refactor this into 'Parsers', which all return a name, a position, a T.
            Token::Kw(Kw::Mod) => {
                let result = parse_mod(&tokens[position..]);
                scope.add_module(&path(result.0), result.1);
                position += result.2;
            }
            Token::Kw(Kw::Class) => {
                let result = parse_class(&tokens[position..]);
                scope.add_class(path(&result.0), result.1);
                position += result.2;
            }
            Token::Kw(Kw::Struct) => {
                let result = parse_struct(&tokens[position..]);
                scope.add_struct(path(&result.0), result.1);
                position += result.2;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }
    }

    scope
}

fn parse_name(token: &LeveledToken, base_level: usize) -> String {
    if let (Token::Global(name), base_level) = token {
        name.clone()
    } else {
        panic!("Expected name")
    }
}

fn parse_mod(tokens: &[LeveledToken]) -> (String, Module, usize) {
    let mut module = Module::new();
    let base_level = tokens[0].1;
    let name = parse_name(&tokens[1], base_level);

    // Skip the 'mod' kw and the module name
    let mut position = 2;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 == base_level {
            break;
        }

        match leveled_token.0 {
            Token::Kw(Kw::Class) => {
                let result = parse_class(&tokens[position..]);
                module.classes.push((result.0, result.1));
                position += result.2;
            }
            Token::Kw(Kw::Struct) => {
                let result = parse_struct(&tokens[position..]);
                module.structs.push((result.0, result.1));
                position += result.2;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }

        position += 1
    }

    (name, module, position)
}

fn parse_traits(tokens: &[LeveledToken]) -> (Vec<Trait>, usize) {
    todo!()
}

fn parse_class(tokens: &[LeveledToken]) -> (String, Class, usize) {
    let mut module = Module::new();
    let base_level = tokens[0].1;
    let name = parse_name(&tokens[1], base_level);
}

fn parse_struct(tokens: &[LeveledToken]) -> (String, Struct, usize) {
    todo!()
}

fn parse_defs(tokens: &[LeveledToken]) -> (Vec<Definition>, usize) {
    todo!()
}

fn parse_lets(tokens: &[LeveledToken]) -> (Vec<Let>, usize) {
    todo!()
}