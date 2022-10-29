use crate::definition::Definition;
use crate::expression::{BinaryCall, BinaryOp, Expression, FriendlyField, LetCall};
use crate::module::Module;
use crate::scope::{path, Scope};
use crate::strukt::Struct;
use crate::trayt::Trait;
use crate::typ::{RawType, Type};

pub fn std_scope() -> Scope {
    let mut scope = Scope::new();

    scope.add_module(&vec!["Op".to_string()], std_op());
    scope.add_module(&vec!["Int".to_string()], std_int());

    scope
}

pub fn std_op() -> Module {
    let mut module = Module::new();

    module.traits.push(
        // Monad like semicolon
        (
            "Then".into(),
            Trait {
                reference_path: path("Op.Then"),
                inputs: vec![Type::Zelf],
                output: Type::Zelf,
            },
        ),
    );

    module.traits.push((
        "Add".into(),
        Trait {
            reference_path: path("Op.Add"),
            inputs: vec![Type::Zelf],
            output: Type::Zelf,
        },
    ));

    module.traits.push((
        "Add".into(),
        Trait {
            reference_path: path("Op.Add"),
            inputs: vec![Type::Zelf],
            output: Type::Zelf,
        },
    ));

    module
}

pub fn std_int() -> Module {
    let mut module = Module::new();

    let mut strukt = Struct::new();

    strukt.add_field("value".into(), RawType::Int);

    strukt.add_definition(
        path("Op.Add"),
        construct_binary_def(BinaryOp::Add, "Int"),
    );

    strukt.add_definition(
        path("Op.Sub"),
        construct_binary_def(BinaryOp::Sub, "Int"),
    );

    strukt.add_definition(
        path("Op.Mul"),
        construct_binary_def(BinaryOp::Mul, "Int"),
    );

    strukt.add_definition(
        path("Op.Div"),
        construct_binary_def(BinaryOp::Div, "Int"),
    );

    module.structs.push(("".into(), strukt));

    module
}

fn construct_binary_def(op: BinaryOp, constructor: &str) -> Definition {
    Definition {
        expression: Expression::Let(LetCall {
            path: path(constructor),
            inputs: [(
                "value".to_string(),
                Expression::Binary(BinaryCall {
                    op,
                    lhs: Box::new(Expression::Local("value".to_string())), // value
                    rhs: Box::new(Expression::FriendlyField(FriendlyField { // rhs.value
                        local_name: "rhs".to_string(),
                        field_name: "value".to_string(),
                    })),
                }),
            )]
                .into(),
        }),
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::expression::{BinaryCall, BinaryOp, Expression, LetCall};
    use crate::instance::Instance;
    use crate::raw_value::RawValue;
    use crate::scope::path;
    use crate::std_lib::std_scope;

    #[test]
    fn test_int_add() {
        let scope = std_scope();

        // Int(value: 1) + Int(value: 2)
        let expression = Expression::Binary(BinaryCall {
            op: BinaryOp::Add,
            lhs: Box::new(Expression::Let(LetCall {
                path: path("Int"),
                inputs: [("value".to_string(), Expression::Literal(RawValue::Int(1)))].into(),
            })),
            rhs: Box::new(Expression::Let(LetCall {
                path: path("Int"),
                inputs: [("value".to_string(), Expression::Literal(RawValue::Int(2)))].into(),
            })),
        });

        let result = expression.resolve(&scope.local_scope(None, HashMap::new()));

        let instance = match result.borrow() {
            Instance::Struct(strukt_instance) => strukt_instance,
            _ => panic!(),
        };

        let value = match instance.value("value") {
            RawValue::Int(value) => value,
            _ => panic!(),
        };

        assert_eq!(*value, 3);
    }

    #[test]
    fn test_int_sub() {
        let scope = std_scope();

        // Int(value: 3) - Int(value: 2)
        let expression = Expression::Binary(BinaryCall {
            op: BinaryOp::Sub,
            lhs: Box::new(Expression::Let(LetCall {
                path: path("Int"),
                inputs: [("value".to_string(), Expression::Literal(RawValue::Int(3)))].into(),
            })),
            rhs: Box::new(Expression::Let(LetCall {
                path: path("Int"),
                inputs: [("value".to_string(), Expression::Literal(RawValue::Int(2)))].into(),
            })),
        });

        let result = expression.resolve(&scope.local_scope(None, HashMap::new()));

        let instance = match result.borrow() {
            Instance::Struct(strukt_instance) => strukt_instance,
            _ => panic!(),
        };

        let value = match instance.value("value") {
            RawValue::Int(value) => value,
            _ => panic!(),
        };

        assert_eq!(*value, 1);
    }
}
