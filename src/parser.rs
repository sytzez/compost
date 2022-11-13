use crate::class::Class;
use crate::definition::Definition;
use crate::expression::{BinaryCall, BinaryOp, DefCall, Expression, FriendlyField, LetCall};
use crate::lett::Let;
use crate::module::Module;
use crate::scope::{path, Scope};
use crate::strukt::Struct;
use crate::token::{Kw, Lit, Op, Token};
use crate::tokenizer::LeveledToken;
use crate::trayt::Trait;
use crate::typ::{RawType, Type};
use crate::RawValue;
use std::borrow::Borrow;
use std::collections::HashMap;

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
            Token::Eof => break,
            _ => panic!("Unexpected token {:?}", leveled_token.0),
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
            Token::Kw(Kw::Defs) => {
                let result = parse_defs(&tokens[position..]);
                for (def_name, def) in result.0 {
                    for (_, class) in module.classes.iter_mut() {
                        class.add_definition(path(&def_name), def.clone());
                    }
                    for (_, strukt) in module.structs.iter_mut() {
                        strukt.add_definition(path(&def_name), def.clone());
                    }

                    // If this definition defines a trait declared in this module, add it to the globally available defs.
                    if module
                        .traits
                        .iter()
                        .any(|(trait_name, _)| (name.clone() + "\\" + trait_name) == def_name)
                    {
                        module.defs.push((def_name, def))
                    }
                }
                position += result.1;
            }
            // TODO: when getting a def, add the def to all classes an strukts
            _ => panic!("Unexpected token {:?}", leveled_token.0),
        }
    }

    (name, module, position)
}

fn parse_class(tokens: &[LeveledToken]) -> (Class, usize) {
    let mut class = Class::new();
    let base_level = tokens[0].1;

    // Skip the 'class' keyword.
    let mut position = 1;

    while position < tokens.len() {
        if tokens[position].1 <= base_level {
            break;
        }

        let result = parse_class_dependency(&tokens[position..]);
        class.add_dependency(result.0, result.1);
        position += result.2;
    }

    (class, position)
}

fn parse_class_dependency(tokens: &[LeveledToken]) -> (String, Type, usize) {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level);
    let type_result = parse_type(&tokens[1..]);
    (name, type_result.0, 1 + type_result.1)
}

fn parse_struct(tokens: &[LeveledToken]) -> (Struct, usize) {
    let mut strukt = Struct::new();
    let base_level = tokens[0].1;

    // Skip the 'struct' keyword.
    let mut position = 1;

    while position < tokens.len() {
        if tokens[position].1 <= base_level {
            break;
        }

        let result = parse_struct_field(&tokens[position..]);
        strukt.add_field(result.0, result.1);
        position += result.2;
    }

    (strukt, position)
}

fn parse_struct_field(tokens: &[LeveledToken]) -> (String, RawType, usize) {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level);
    let typ_name = parse_local(&tokens[1], base_level + 1);
    let typ = match typ_name.borrow() {
        "int" => RawType::Int,
        "uint" => RawType::UInt,
        "string" => RawType::String,
        _ => panic!("Unknown raw type {}", typ_name),
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
            Token::Kw(Kw::Zelf) => {
                output = Some(Type::Zelf);
                position += 1;

                break;
            }
            Token::Local(_) => {
                let result = parse_parameter(&tokens[position..]);
                inputs.push(result.1); // TODO: add names to Trait inputs.
                position += result.2;
            }
            Token::Op(Op::Sub) => {
                assert_eq!(
                    &tokens[position + 1].0,
                    &Token::Op(Op::Gt),
                    "Expected > after -"
                );

                let result = parse_type(&tokens[position + 2..]);
                output = Some(result.0);
                position += 2 + result.1;

                break;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0),
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

    assert!(
        tokens[1].1 > base_level,
        "Expected type after parameter name"
    );
    let type_result = parse_type(&tokens[1..]);

    (name, type_result.0, type_result.1 + 1)
}

fn parse_defs(tokens: &[LeveledToken]) -> (Vec<(String, Definition)>, usize) {
    let base_level = tokens[0].1;
    let mut defs = vec![];

    // Skip the 'defs' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        match &leveled_token.0 {
            Token::Global(name) => {
                position += 1;

                let result = parse_expression(&tokens[position..]);
                defs.push((
                    name.clone(),
                    Definition {
                        expression: result.0,
                    },
                ));
                position += result.1;
            }
            // TODO: nesting
            _ => panic!("Unexpected token {:?}", leveled_token.0),
        }
    }

    (defs, position)
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
                assert_eq!(
                    &tokens[position + 1].0,
                    &Token::Op(Op::Gt),
                    "Expected > after -"
                );

                let result = parse_type(&tokens[position + 2..]);
                output = Some(result.0);
                position += 2 + result.1;

                break;
            }
            _ => panic!("Unexpected token {:?}", leveled_token.0),
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
    let base_level = tokens[0].1;
    let mut position = 0;

    // First token
    let mut expression = match &tokens[0].0 {
        Token::Kw(kw) => match kw {
            Kw::Zelf => {
                position += 1;
                Expression::Zelf
            }
            _ => panic!("Unexpected keyword {:?}", kw),
        },
        Token::Global(_) => {
            let result = parse_let_call(&tokens[position..]);
            position += result.1;
            Expression::Let(result.0)
        }
        Token::Local(name) => {
            position += 1;
            Expression::Local(name.clone())
        }
        Token::Lit(lit) => {
            position += 1;
            Expression::Literal(match lit {
                Lit::String(value) => RawValue::String(value.clone()),
                Lit::Number(value) => RawValue::UInt(*value as u64),
            })
        }
        Token::Op(Op::Dot) => {
            // We don't increase the position to reevaluate the dot in the next step
            Expression::Zelf
        }
        // TODO: .Def (meaning use Self as subject)
        _ => panic!("Unexpected token {:?}", tokens[0].0),
    };

    // Further operations
    while position < tokens.len() {
        if tokens[position].1 < base_level {
            break;
        }

        expression = match &tokens[position].0 {
            Token::Op(op) => match op {
                Op::Add | Op::Sub | Op::Mul | Op::Div => {
                    position += 1;

                    let result = parse_expression(&tokens[position..]);
                    position += result.1;

                    Expression::Binary(BinaryCall {
                        op: match op {
                            Op::Add => BinaryOp::Add,
                            Op::Sub => BinaryOp::Sub,
                            Op::Mul => BinaryOp::Mul,
                            Op::Div => BinaryOp::Div,
                            _ => unreachable!(),
                        },
                        lhs: Box::new(expression),
                        rhs: Box::new(result.0),
                    })
                }
                Op::Dot => {
                    position += 1;

                    match (expression, &tokens[position].0) {
                        (Expression::Local(local_name), Token::Local(field_name)) => {
                            position += 1;

                            Expression::FriendlyField(FriendlyField {
                                local_name,
                                field_name: field_name.clone(),
                            })
                        }
                        (expression, Token::Global(_)) => {
                            let result = parse_let_call(&tokens[position..]);
                            position += result.1;

                            Expression::Def(DefCall {
                                path: result.0.path,
                                subject: Box::new(expression),
                                inputs: result.0.inputs,
                            })
                        }
                        _ => panic!(
                            "Dot operator must be followed by a trait or friendly field name"
                        ),
                    }
                }
                _ => panic!("Unexpected operator {:?}", op),
            },
            _ => panic!("Unexpected token {:?}", tokens[position].0),
        };
    }

    (expression, position)
}

// e.g.: Module\Function(param1: ..., param2: ...)
// e.g.: Module\Constant
fn parse_let_call(tokens: &[LeveledToken]) -> (LetCall, usize) {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level);
    let mut inputs = HashMap::new();

    // Skip the name of the let call.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        match &leveled_token.0 {
            Token::Local(param_name) => {
                position += 1;

                let result = parse_expression(&tokens[position..]);
                inputs.insert(param_name.clone(), result.0);
                position += result.1;
            }
            _ => break,
        }
    }

    let let_call = LetCall {
        path: path(&name),
        inputs,
    };

    (let_call, position)
}

// TODO: test everything
