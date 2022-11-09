use std::borrow::Borrow;
use std::collections::HashMap;
use crate::class::Class;
use crate::definition::Definition;
use crate::expression::Expression;
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
                scope.add_module(&path(&result.0), result.1);
                position += result.2;
            }
            Token::Kw(Kw::Lets) => {
                let result = parse_lets(&tokens[position..]);

                for (name, lett) in result.0 {
                    scope.add_let(path(&name), lett)
                }

                position += result.1;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0)
        }
    }

    scope
}

fn parse_global(token: &LeveledToken, expected_level: usize) -> String {
    if let (Token::Global(name), actual_level) = token {
        assert_eq!(*actual_level, expected_level);
        name.clone()
    } else {
        panic!("Expected global name, got {:?} ", token)
    }
}

fn parse_local(token: &LeveledToken, expected_level: usize) -> String {
    if let (Token::Local(name), actual_level) = token {
        assert_eq!(*actual_level, expected_level);
        name.clone()
    } else {
        panic!("Expected local name, got {:?} ", token)
    }
}

fn parse_mod(tokens: &[LeveledToken]) -> (String, Module, usize) {
    println!("mod");

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
                module.classes.push(("".to_string(), result.0));
                position += result.1;
            }
            Token::Kw(Kw::Struct) => {
                let result = parse_struct(&tokens[position..]);
                module.structs.push(("".to_string(), result.0));
                position += result.1;
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

fn parse_class(tokens: &[LeveledToken]) -> (Class, usize) {
    todo!()
}

fn parse_struct(tokens: &[LeveledToken]) -> (Struct, usize) {
    println!("struct");

    let mut strukt = Struct::new();
    let base_level = tokens[0].1;

    // Skip the 'struct' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_struct_field(&tokens[position..]);
        strukt.add_field(result.0, result.1);
        position += result.2;
    }

    (strukt, position)
}

fn parse_struct_field(tokens: &[LeveledToken]) -> (String, RawType, usize) {
    println!("struct field");

    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level);
    let typ_name = parse_global(&tokens[1], base_level + 1);
    let typ = match typ_name.borrow() {
        "Int" => RawType::Int,
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

fn parse_lets(tokens: &[LeveledToken]) -> (Vec<(String, Let)>, usize) {
    let base_level = tokens[0].1;
    let mut lets = vec![];

    // Skip the 'lets' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_let(&tokens[position..]);
        lets.push((result.0, result.1));
        position += result.2;
    }

    (lets, position)
}

fn parse_let(tokens: &[LeveledToken]) -> (String, Let, usize) {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level);
    let mut inputs = HashMap::new();
    let mut output = None;

    // Skip the name of the let.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
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
                inputs.insert(result.0, result.1);
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

    let result = parse_expression(&tokens[position..]);
    let expression = result.0;
    position += result.1;

    let lett = Let {
        inputs,
        outputs: [("".into(), output.unwrap())].into(),
        expression,
    };

    (name, lett, position)
}

fn parse_expression(tokens: &[LeveledToken]) -> (Expression, usize) {
    todo!()
}

// TODO: test everything

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::rc::Rc;
    use crate::instance::Instance;
    use crate::parser::parse_tokens;
    use crate::raw_value::RawValue;
    use crate::scope::path;
    use crate::token::{Kw, Token};

    #[test]
    fn test_struct() {
        let tokens = [
            // mod StructName
            (Token::Kw(Kw::Mod), 0),
            (Token::Global("StructName".into()), 0),
            // struct
            (Token::Kw(Kw::Struct), 1),
            // x: Int
            (Token::Local("x".into()), 2),
            (Token::Global("Int".into()), 3),
            // y: Int
            (Token::Local("y".into()), 2),
            (Token::Global("Int".into()), 3),
            // traits
            (Token::Kw(Kw::Traits), 1),
            // X: Int
            (Token::Global("X".into()), 2),
            (Token::Global("Int".into()), 3),
            // Y: Int
            (Token::Global("Y".into()), 2),
            (Token::Global("Int".into()), 3),
            // defs
            // (Token::Kw(Kw::Defs), 1),
            // // X: x
            // (Token::Global("X".into()), 2),
            // (Token::Local("x".into()), 3),
            // // Y: y
            // (Token::Global("Y".into()), 2),
            // (Token::Local("y".into()), 3),
        ];

        let scope = parse_tokens(&tokens);

        let module = scope.lett(&path("StructName"));

        let local_scope = scope.local_scope(None, [
            ("x".to_string(), Rc::new(Instance::Raw(RawValue::Int(1)))),
            ("y".to_string(), Rc::new(Instance::Raw(RawValue::Int(2)))),
        ].into());

        let strukt = module.expression.resolve(&local_scope);

        if let Instance::Struct(strukt_instance) = strukt.borrow() {
            assert_eq!(
                strukt_instance.values,
                [
                    ("x".to_string(), RawValue::Int(1)),
                    ("y".to_string(), RawValue::Int(2)),
                ].into()
            );
        } else {
            panic!("Instance is not a struct")
        }
    }
}
