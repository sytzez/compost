use crate::ast::expression::{BinaryCall, BinaryOp, DefCall, Expression, FriendlyField, LetCall};
use crate::ast::raw_value::RawValue;
use crate::ast::typ::{RawType, Type};
use crate::error::{error, CResult};
use crate::lex::token::{Kw, Lit, Op, Token};
use crate::lex::tokenizer::LeveledToken;
use crate::sem::class::Class;
use crate::sem::definition::Definition;
use crate::sem::lett::Let;
use crate::sem::module::Module;
use crate::sem::scope::{path, Scope};
use crate::sem::strukt::Struct;
use crate::sem::trayt::Trait;
use std::borrow::Borrow;
use std::collections::HashMap;

// Parse a series of leveled tokens into a scope.
pub fn parse_tokens(tokens: &[LeveledToken]) -> CResult<Scope> {
    let mut position = 0;

    let mut scope = Scope::new();

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 != 0 {
            return error(format!("Unexpected level {}, expected 0", leveled_token.1), 0);
        }

        match leveled_token.0 {
            // TODO: refactor this into 'Parsers', which all return a name, a position, a T.
            Token::Kw(Kw::Mod) => {
                let result = parse_mod(&tokens[position..])?;
                scope.add_module(&path(&result.0), result.1);
                position += result.2;
            }
            Token::Kw(Kw::Lets) => {
                let result = parse_lets(&tokens[position..])?;

                for (name, lett) in result.0 {
                    scope.add_let(path(&name), lett)
                }

                position += result.1;
            }
            Token::Eof => break,
            _ => return error(format!("Unexpected token {:?}", leveled_token.0), position),
        }
    }

    Ok(scope)
}

fn parse_global(token: &LeveledToken, expected_level: usize) -> CResult<String> {
    if let (Token::Global(name), actual_level) = token {
        if *actual_level != expected_level {
            error("Unexpected code level for global name".to_string(), 0)
        } else {
            Ok(name.clone())
        }
    } else {
        error(format!("Expected global name, got {:?} ", token), 0)
    }
}

fn parse_local(token: &LeveledToken, expected_level: usize) -> CResult<String> {
    if let (Token::Local(name), actual_level) = token {
        if *actual_level != expected_level {
            error("Unexpected code level for local name".to_string(), 0)
        } else {
            Ok(name.clone())
        }
    } else {
        error(format!("Expected local name, got {:?} ", token), 0)
    }
}

fn parse_mod(tokens: &[LeveledToken]) -> CResult<(String, Module, usize)> {
    let mut module = Module::new();
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[1], base_level)?;

    // Skip the 'mod' keyword and the module name.
    let mut position = 2;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        match leveled_token.0 {
            Token::Kw(Kw::Class) => {
                let result = parse_class(&tokens[position..])?;
                module.classes.push(("".to_string(), result.0));
                position += result.1;
            }
            Token::Kw(Kw::Struct) => {
                let result = parse_struct(&tokens[position..])?;
                module.structs.push(("".to_string(), result.0));
                position += result.1;
            }
            Token::Kw(Kw::Traits) => {
                let result = parse_traits(&tokens[position..])?;
                for (name, trayt) in result.0 {
                    module.traits.push((name, trayt));
                }
                position += result.1;
            }
            Token::Kw(Kw::Defs) => {
                let result = parse_defs(&tokens[position..])?;
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
            _ => return error(format!("Unexpected token {:?}", leveled_token.0), 0),
        }
    }

    Ok((name, module, position))
}

fn parse_class(tokens: &[LeveledToken]) -> CResult<(Class, usize)> {
    let mut class = Class::new();
    let base_level = tokens[0].1;

    // Skip the 'class' keyword.
    let mut position = 1;

    while position < tokens.len() {
        if tokens[position].1 <= base_level {
            break;
        }

        let result = parse_class_dependency(&tokens[position..])?;
        class.add_dependency(result.0, result.1);
        position += result.2;
    }

    Ok((class, position))
}

fn parse_class_dependency(tokens: &[LeveledToken]) -> CResult<(String, Type, usize)> {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level)?;
    let type_result = parse_type(&tokens[1..])?;
    Ok((name, type_result.0, 1 + type_result.1))
}

fn parse_struct(tokens: &[LeveledToken]) -> CResult<(Struct, usize)> {
    let mut strukt = Struct::new();
    let base_level = tokens[0].1;

    // Skip the 'struct' keyword.
    let mut position = 1;

    while position < tokens.len() {
        if tokens[position].1 <= base_level {
            break;
        }

        let result = parse_struct_field(&tokens[position..])?;
        strukt.add_field(result.0, result.1);
        position += result.2;
    }

    Ok((strukt, position))
}

fn parse_struct_field(tokens: &[LeveledToken]) -> CResult<(String, RawType, usize)> {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level)?;
    let typ_name = parse_local(&tokens[1], base_level + 1)?;
    let typ = match typ_name.borrow() {
        "int" => RawType::Int,
        "string" => RawType::String,
        _ => return error(format!("Unknown struct field type {}", typ_name), 0),
    };
    Ok((name, typ, 2))
}

fn parse_traits(tokens: &[LeveledToken]) -> CResult<(Vec<(String, Trait)>, usize)> {
    let mut traits = vec![];
    let base_level = tokens[0].1;

    // Skip the 'traits' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_trait(&tokens[position..])?;
        traits.push((result.0, result.1));
        position += result.2;
    }

    Ok((traits, position))
}

fn parse_trait(tokens: &[LeveledToken]) -> CResult<(String, Trait, usize)> {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level)?;
    let mut inputs = vec![];
    let mut output = None;

    // Skip trait name.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            return error("Expected trait types".to_string(), 0);
        }

        match leveled_token.0 {
            Token::Global(_) => {
                let result = parse_type(&tokens[position..])?;
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
                let result = parse_parameter(&tokens[position..])?;
                inputs.push(result.1); // TODO: add names to Trait inputs.
                position += result.2;
            }
            Token::Op(Op::Sub) => {
                assert_eq!(
                    &tokens[position + 1].0,
                    &Token::Op(Op::Gt),
                    "Expected > after -"
                );

                let result = parse_type(&tokens[position + 2..])?;
                output = Some(result.0);
                position += 2 + result.1;

                break;
            }
            _ => return error(format!("Unexpected token {:?}", leveled_token.0), 0),
        }
    }

    let trayt = Trait {
        inputs,
        output: output.unwrap(),
    };

    Ok((name, trayt, position))
}

fn parse_type(tokens: &[LeveledToken]) -> CResult<(Type, usize)> {
    // TODO: get trait paths, handle & and |, handle Self, handle Void
    Ok((Type::Zelf, 1))
}

fn parse_parameter(tokens: &[LeveledToken]) -> CResult<(String, Type, usize)> {
    let base_level = tokens[0].1;
    let name = parse_local(&tokens[0], base_level)?;

    if tokens[1].1 <= base_level {
        return error("Expected type after parameter name".to_string(), 0);
    }
    let type_result = parse_type(&tokens[1..])?;

    Ok((name, type_result.0, type_result.1 + 1))
}

fn parse_defs(tokens: &[LeveledToken]) -> CResult<(Vec<(String, Definition)>, usize)> {
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

                let result = parse_expression(&tokens[position..])?;
                defs.push((
                    name.clone(),
                    Definition {
                        expression: result.0,
                    },
                ));
                position += result.1;
            }
            // TODO: nesting
            _ => return error(format!("Unexpected token {:?}", leveled_token.0), 0),
        }
    }

    Ok((defs, position))
}

fn parse_lets(tokens: &[LeveledToken]) -> CResult<(Vec<(String, Let)>, usize)> {
    let base_level = tokens[0].1;
    let mut lets = vec![];

    // Skip the 'lets' keyword.
    let mut position = 1;

    while position < tokens.len() {
        let leveled_token = &tokens[position];

        if leveled_token.1 <= base_level {
            break;
        }

        let result = parse_let(&tokens[position..])?;
        lets.push((result.0, result.1));
        position += result.2;
    }

    Ok((lets, position))
}

fn parse_let(tokens: &[LeveledToken]) -> CResult<(String, Let, usize)> {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level)?;
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
                let result = parse_type(&tokens[position..])?;
                output = Some(result.0);
                position += result.1;

                break;
            }
            Token::Local(_) => {
                let result = parse_parameter(&tokens[position..])?;
                inputs.insert(result.0, result.1);
                position += result.2;
            }
            Token::Op(Op::Sub) => {
                assert_eq!(
                    &tokens[position + 1].0,
                    &Token::Op(Op::Gt),
                    "Expected > after -"
                );

                let result = parse_type(&tokens[position + 2..])?;
                output = Some(result.0);
                position += 2 + result.1;

                break;
            }
            _ => return error(format!("Unexpected token {:?}", leveled_token.0), 0),
        }
    }

    let result = parse_expression(&tokens[position..])?;
    let expression = result.0;
    position += result.1;

    let lett = Let {
        inputs,
        outputs: [("".into(), output.unwrap())].into(),
        expression,
    };

    Ok((name, lett, position))
}

fn parse_expression(tokens: &[LeveledToken]) -> CResult<(Expression, usize)> {
    let base_level = tokens[0].1;
    let mut position = 0;

    // First token
    let mut expression = match &tokens[0].0 {
        Token::Kw(kw) => match kw {
            Kw::Zelf => {
                position += 1;
                Expression::Zelf
            }
            _ => return error(format!("Unexpected keyword {:?}", kw), 0),
        },
        Token::Global(_) => {
            let result = parse_let_call(&tokens[position..])?;
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
                Lit::Number(value) => RawValue::Int(*value as i64),
            })
        }
        Token::Op(Op::Dot) => {
            // We don't increase the position to reevaluate the dot in the next step
            Expression::Zelf
        }
        Token::Op(Op::Sub) => {
            position += 1;

            let result = parse_expression(&tokens[position..])?;
            position += result.1;

            Expression::Def(DefCall {
                path: path("Op\\Neg"),
                subject: Box::new(result.0),
                inputs: [].into(),
            })
        }
        _ => return error(format!("Unexpected token {:?}", tokens[0].0), 0),
    };

    // Further operations
    while position < tokens.len() {
        if tokens[position].1 < base_level {
            break;
        }

        expression = match &tokens[position].0 {
            Token::Op(op) => {
                match op {
                    Op::Add | Op::Sub | Op::Mul | Op::Div => {
                        position += 1;

                        let result = parse_expression(&tokens[position..])?;
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
                                let result = parse_let_call(&tokens[position..])?;
                                position += result.1;

                                Expression::Def(DefCall {
                                    path: result.0.path,
                                    subject: Box::new(expression),
                                    inputs: result.0.inputs,
                                })
                            }
                            _ => return error(
                                "Dot operator must be followed by a trait or friendly field name"
                                    .to_string(), 0
                            ),
                        }
                    }
                    _ => return error(format!("Unexpected operator {:?}", op), 0),
                }
            }
            _ => return error(format!("Unexpected token {:?}", tokens[position].0), 0),
        };
    }

    Ok((expression, position))
}

// e.g.: Module\Function(param1: ..., param2: ...)
// e.g.: Module\Constant
fn parse_let_call(tokens: &[LeveledToken]) -> CResult<(LetCall, usize)> {
    let base_level = tokens[0].1;
    let name = parse_global(&tokens[0], base_level)?;
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

                let result = parse_expression(&tokens[position..])?;
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

    Ok((let_call, position))
}

// TODO: test everything
