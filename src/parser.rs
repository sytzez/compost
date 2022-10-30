use std::borrow::Borrow;
use crate::class::Class;
use crate::definition::Definition;
use crate::lett::Let;
use crate::module::Module;
use crate::scope::{path, Scope};
use crate::strukt::Struct;
use crate::token::{Kw, Op, Token};
use crate::tokenizer::LeveledToken;
use crate::trayt::Trait;
use crate::typ::{RawType, Type};

// Parse a series of leveled tokens into a scope.
pub fn parse_tokens(tokens: &[LeveledToken]) -> Scope {
    let mut position = 0;

    let mut scope = Scope::new();

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 != 0 {
            panic!("Unexpected level {}, expected 0", leveled_token.1)
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
            Token::Kw(Kw::Traits) => {
                let result = parse_traits(&tokens[position..]);
                for (name, trayt) in result.0 {
                    scope.add_trait(path(&name), trayt);
                }
                position += result.1;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }
    }

    scope
}

fn parse_global(token: &LeveledToken, base_level: usize) -> String {
    if let (Token::Global(name), base_level) = token {
        name.clone()
    } else {
        panic!("Expected global name")
    }
}

fn parse_local(token: &LeveledToken, base_level: usize) -> String {
    if let (Token::Local(name), base_level) = token {
        name.clone()
    } else {
        panic!("Expected local name")
    }
}

fn parse_mod(tokens: &[LeveledToken]) -> (String, Module, usize) {
    let mut module = Module::new();
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[1], base_level);

    // Skip the 'mod' keyword and the module name.
    let mut position = 2;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
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
            Token::Kw(Kw::Traits) => {
                let result = parse_traits(&tokens[position..]);
                for (name, trayt) in result.0 {
                    module.traits.push((name, trayt));
                }
                position += result.1;
            }
            // TODO: when getting a def, add the def to all classes an strukts
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }
    }

    (name, module, position)
}

fn parse_class(tokens: &[LeveledToken]) -> (String, Class, usize) {
    todo!()
}

fn parse_struct(tokens: &[LeveledToken]) -> (String, Struct, usize) {
    let mut strukt = Struct::new();
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[1], base_level);

    // Skip the 'struct' keyword and the struct name.
    let mut position = 2;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_struct_field(&tokens[position..]);
        strukt.add_field(result.0, result.1);
        position += result.2;
    }

    (name, strukt, position)
}

fn parse_struct_field(tokens: &[LeveledToken]) -> (String, RawType, usize) {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level);
    let typ_name = parse_global(&tokens[1], base_level + 1);
    let typ = match typ_name.borrow() {
        "Int" => RawType::Int,
        "Float" => RawType::Float,
        "UInt" => RawType::UInt,
        "String" => RawType::String,
        _ => panic!("Unknown raw type {}", typ_name)
    };
    (name, typ, 2)
}

fn parse_traits(tokens: &[LeveledToken]) -> (Vec<(String, Trait)>, usize) {
    let mut traits = vec![];
    let base_level = tokens[0].1;

    // Skip the 'traits' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_trait(&tokens[position..]);
        traits.push((result.0, result.1));
        position += result.2;
    }

    (traits, position)
}

fn parse_trait(tokens: &[LeveledToken]) -> (String, Trait, usize) {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level);
    let mut inputs = vec![];
    let mut output = None;

    // Skip trait name.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            panic!("Expected trait types");
        }

        match leveled_token.0 {
            Token::Global(_) => {
                let result = parse_type(&tokens[position..]);
                output = Some(result.0);
                position += result.1;

                break;
            }
            Token::Local(_) => {
                let result = parse_parameter(&tokens[position..]);
                inputs.push(result.1); // TODO: add names to Trait inputs.
                position += result.2;
            }
            Token::Op(Op::Sub) => {
                assert_eq!(&tokens[position + 1].0, &Token::Op(Op::Gt), "Expected > after -");

                let result = parse_type(&tokens[position+2..]);
                output = Some(result.0);
                position += 2 + result.1;

                break;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }
    }

    let trayt = Trait {
        inputs,
        output: output.unwrap(),
    };

    (name, trayt, position)
}

fn parse_type(tokens: &[LeveledToken]) -> (Type, usize) {
    // TODO: get trait paths, handle & and |, handle Self, handle Void
    (Type::Zelf, 1)
}

fn parse_parameter(tokens: &[LeveledToken]) -> (String, Type, usize) {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level);

    assert!(tokens[1].1 > base_level, "Expected type after parameter name");
    let type_result = parse_type(&tokens[1..]);

    (name, type_result.0, type_result.1 + 1)
}

fn parse_defs(tokens: &[LeveledToken]) -> (Vec<Definition>, usize) {
    todo!()
}

fn parse_lets(tokens: &[LeveledToken]) -> (Vec<Let>, usize) {
    todo!()
}

// TODO: test everything