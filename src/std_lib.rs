use crate::class::{Definition, Instance, RawType, RawValue, Struct, StructInstance, Trait, Type};
use crate::expression::{Expression, DefCall};
use crate::module::Module;
use crate::scope::{create_reference_path, Scope};

pub fn std_scope() -> Scope {
    let mut scope = Scope::new();

    scope.add_module(&vec!["Op".to_string()], std_op());
    scope.add_module(&vec!["Int".to_string()], std_int());

    scope
}

pub fn std_op() -> Module {
    let mut module = Module::new();

    module.traits.push(
        ("Add".into(), Trait {
            reference_path: create_reference_path("Op.Add"),
            inputs: vec![Type::Zelf],
            outputs: vec![Type::Zelf],
        })
    );

    module.traits.push(
        ("Add".into(), Trait {
            reference_path: create_reference_path("Op.Add"),
            inputs: vec![Type::Zelf],
            outputs: vec![Type::Zelf],
        })
    );

    module
}

pub fn std_int() -> Module {
    let mut module = Module::new();

    let mut strukt = Struct::new();

    strukt.add_field("value".into(), RawType::Int);

    strukt.add_definition(
        create_reference_path("Op.Add"),
        Definition {
            expression: Expression::Def(DefCall {
                path: create_reference_path("Op.Add"),
                inputs: [("rhs".to_string(), Expression::Local("rhs".to_string()))].into(),
            }),
        }
    );

    module.structs.push(("".into(), strukt));

    module
}

#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use std::collections::HashMap;

    use crate::class::{Instance, RawValue};
    use crate::expression::{Expression, LetCall, BinaryOp, BinaryCall};
    use crate::scope::create_reference_path;
    use crate::std_lib::std_scope;

    #[test]
    fn test_int_add() {
        let scope = std_scope();

        // Int(1) + Int(2)
        let expression = Expression::Binary(
            BinaryCall {
                op: BinaryOp::Add,
                lhs: Box::new(Expression::Let(
                    LetCall {
                        path: create_reference_path("Int"),
                        inputs: [("value".to_string(), Expression::Literal(RawValue::Int(1)))].into(),
                    }
                )),
                rhs: Box::new(Expression::Let(
                    LetCall {
                        path: create_reference_path("Int"),
                        inputs: [("value".to_string(), Expression::Literal(RawValue::Int(2)))].into(),
                    }
                )),
            }
        );

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
