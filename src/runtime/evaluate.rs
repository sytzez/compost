use crate::runtime::class_instance::ClassInstance;
use crate::runtime::instance::Instance;
use crate::runtime::struct_instance::StructInstance;
use crate::sem::evaluation::Evaluation;
use std::collections::HashMap;
use std::rc::Rc;

/// Evaluate an evaluation into an instance.
pub fn evaluate(
    eval: &Evaluation,
    locals: &HashMap<String, Rc<Instance>>,
    zelf: &Option<Rc<Instance>>,
) -> Rc<Instance> {
    match eval {
        Evaluation::Let(call) => {
            let locals = evaluate_inputs(&call.inputs, locals, zelf);

            let eval = &call.lett.borrow().evaluation;

            evaluate(eval, &locals, &None)
        }
        Evaluation::Trait(call) => {
            let subject = evaluate(&call.subject, locals, zelf);

            let inputs = evaluate_inputs(&call.inputs, locals, zelf);

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
                dbg!(instance);
                panic!(
                    "Friendly field does not exist: {}.{}",
                    ff.local_name, ff.field_name
                )
            }
        }
        Evaluation::Match(call) => {
            let subject = evaluate(&call.subject, locals, &zelf);

            // Check if the subject is of the same type as 'self' in this scope
            let is_self = match &zelf {
                Some(zelf) => zelf.is_of_same_type(&subject),
                None => false,
            };

            // Find the matching branch.
            let branch = call
                .branches
                .iter()
                .find(|(typ, _)| subject.satisfies_type(typ, is_self))
                .map(|(_, branch)| branch)
                .unwrap_or_else(|| {
                    unreachable!("None of the branches for {} matched!", call.local_name)
                });

            // Add the subject to scope.
            let locals = locals
                .clone()
                .into_iter()
                .chain([(call.local_name.to_string(), subject)])
                .collect();

            evaluate(branch, &locals, zelf)
        }
        Evaluation::IfElse(call) => {
            let condition = evaluate(&call.condition, locals, zelf);

            if condition.to_bool() {
                evaluate(&call.iff, locals, zelf)
            } else {
                evaluate(&call.els, locals, zelf)
            }
        }
        Evaluation::Zelf => Rc::clone(zelf.as_ref().unwrap()),
        Evaluation::Void => Rc::new(Instance::Void),
        Evaluation::ClassConstructor(class) => {
            let instance = ClassInstance::new(class, locals.clone());

            Rc::new(Instance::Class(instance))
        }
        Evaluation::StructConstructor(strukt) => {
            let fields = locals
                .iter()
                .map(|(name, instance)| {
                    if let Instance::Raw(value) = instance.as_ref() {
                        (name.to_string(), value.clone())
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

fn evaluate_inputs(
    inputs: &[(String, Evaluation)],
    locals: &HashMap<String, Rc<Instance>>,
    zelf: &Option<Rc<Instance>>,
) -> HashMap<String, Rc<Instance>> {
    inputs
        .iter()
        .map(|(name, eval)| (name.clone(), evaluate(eval, locals, zelf)))
        .collect()
}
