use std::collections::HashMap;
use crate::class::{Definition, Instance, RawType, RawValue, Struct, StructInstance, Trait, Type};
use crate::module::Module;
use crate::scope::{create_reference_path, Scope};

pub fn std_scope() -> Scope {
    let mut scope = Scope::new();

    scope.add_module("Op", std_op());
    scope.add_module("Int", std_int());

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
            call: int_add,
        }
    );

    module.structs.push(("".into(), strukt));

    module
}

fn int_add(left: &Instance, inputs: HashMap<String, &Instance>) -> HashMap<String, Instance> {
    let left = match left {
        Instance::Struct(instance) => instance,
        _ => panic!(),
    };

    let right = match inputs.get("rhs").unwrap() {
        Instance::Struct(instance) => instance,
        _ => panic!(),
    };

    let left_value = match left.value("value") {
        RawValue::Int(value) => value,
        _ => panic!(),
    };

    let right_value = match right.value("value") {
        RawValue::Int(value) => value,
        _ => panic!(),
    };

    let result = left_value + right_value;

    let mut result_struct = StructInstance::new(left.strukt());

    result_struct.set_value("value".into(), RawValue::Int(result));

    [("".to_string(), Instance::Struct(result_struct))].into()
}

#[cfg(test)]
mod test {
    use crate::class::{Instance, RawValue};
    use crate::scope::create_reference_path;
    use crate::std_lib::std_scope;

    #[test]
    fn test_int_add() {
        let scope = std_scope();

        let int_constructor = scope.lett("Int");

        // Call int constructor with '1'
        let raw_int_1 = Instance::Raw(RawValue::Int(1));
        let int_1_inputs = [("value".to_string(), &raw_int_1)].into();
        let int_1_outputs = (int_constructor.call)(int_1_inputs);
        let int_1 = int_1_outputs.get("").unwrap();

        // Call int constructor with '2'
        let raw_int_2 = Instance::Raw(RawValue::Int(2));
        let int_2_inputs = [("value".to_string(), &raw_int_2)].into();
        let int_2_outputs = (int_constructor.call)(int_2_inputs);
        let int_2 = int_2_outputs.get("").unwrap();

        // Call Op.Add trait on int_1 and int_2
        let int_3_inputs = [("rhs".to_string(), int_2)].into();
        let int_3_outputs = int_1.call(&create_reference_path("Op.Add"), int_3_inputs);
        let int_3 = int_3_outputs.get("").unwrap();

        let instance = match int_3 {
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
