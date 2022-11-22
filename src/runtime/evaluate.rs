use crate::runtime::instance::Instance;
use crate::sem::evaluation::Evaluation;
use std::collections::HashMap;
use std::rc::Rc;
use crate::runtime::class_instance::ClassInstance;
use crate::runtime::struct_instance::StructInstance;

pub fn evaluate(
    eval: &Evaluation,
    locals: HashMap<String, Rc<Instance>>,
    zelf: Option<Rc<Instance>>,
) -> Rc<Instance> {
    match eval {
        Evaluation::Let(call) => {
            let locals = evaluate_inputs(&call.inputs, &locals, &zelf);

            let eval = &call.lett.borrow().evaluation;

            evaluate(eval, locals, None)
        }
        Evaluation::Trait(call) => {
            let subject = evaluate(&call.subject, locals.clone(), zelf.clone());

            let inputs = evaluate_inputs(&call.inputs, &locals, &zelf);

            subject.call(Rc::clone(&call.trayt), inputs)
        }
        Evaluation::Literal(value) => Rc::new(Instance::Raw(value.clone())),
        Evaluation::Local(name) => Rc::clone(locals.get(name).unwrap()),
        Evaluation::FriendlyField(ff) => {
            let instance = locals.get(&ff.local_name).unwrap();

            if let Instance::Struct(strukt) = instance.as_ref() {
                let value = strukt.field(&ff.field_name);

                Rc::new(Instance::Raw(value.clone()))
            } else {
                unreachable!()
            }
        }
        Evaluation::Zelf => Rc::clone(&zelf.unwrap()),
        Evaluation::ClassConstructor(class) => {
            let instance = ClassInstance::new(class, locals);

            Rc::new(Instance::Class(instance))
        }
        Evaluation::StructConstructor(strukt) => {
            let fields = locals
                .into_iter()
                .map(|(name, instance)| {
                    if let Instance::Raw(value) = instance.as_ref() {
                        (name, value.clone())
                    } else {
                        unreachable!()
                    }
                })
                .collect();

            let instance = StructInstance::new(strukt, fields);

            Rc::new(Instance::Struct(instance))
        }
    }
}

pub fn evaluate_inputs(inputs: &Vec<(String, Evaluation)>, locals: &HashMap<String, Rc<Instance>>, zelf: &Option<Rc<Instance>>) -> HashMap<String, Rc<Instance>> {
    inputs.iter()
        .map(|(name, eval)| (
            name.clone(),
            evaluate(eval, locals.clone(), zelf.clone())),
        )
        .collect()
}