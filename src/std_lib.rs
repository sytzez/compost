use crate::class::{Definition, Instance, RawType, RawValue, Struct, StructInstance, Trait, Type};
use crate::expression::{BinaryCall, BinaryOp, DefCall, Expression, FriendlyField, LetCall};
use crate::module::Module;
use crate::scope::{path, Scope};

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
                outputs: vec![Type::Zelf],
            },
        ),
    );

    module.traits.push((
        "Add".into(),
        Trait {
            reference_path: path("Op.Add"),
            inputs: vec![Type::Zelf],
            outputs: vec![Type::Zelf],
        },
    ));

    module.traits.push((
        "Add".into(),
        Trait {
            reference_path: path("Op.Add"),
            inputs: vec![Type::Zelf],
            outputs: vec![Type::Zelf],
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
        Definition {
            expression: Expression::Let(LetCall {
                path: path("Int"),
                inputs: [(
                    "value".to_string(),
                    Expression::Binary(BinaryCall {
                        op: BinaryOp::Add,
                        lhs: Box::new(Expression::Local("value".to_string())),
                        rhs: Box::new(Expression::FriendlyField(FriendlyField {
                            local_name: "rhs".to_string(),
                            field_name: "value".to_string(),
                        })),
                    }),
                )]
                .into(),
            }),
        },
    );

    module.structs.push(("".into(), strukt));

    module
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::class::{Instance, RawValue};
    use crate::expression::{BinaryCall, BinaryOp, Expression, LetCall};
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
}
