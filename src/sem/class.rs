use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::module_statement::ModuleStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::{interface_type, Trait};
use crate::sem::typ::{combine_types, Type};
use crate::sem::type_coercion::coerce_type;
use std::rc::Rc;
use std::string::String;
use std::sync::atomic::{AtomicUsize, Ordering};

static ID: AtomicUsize = AtomicUsize::new(0);

// A class has a set of dependencies of certain types, and a set of trait definitions.
#[derive(Debug)]
pub struct Class {
    pub dependencies: Vec<(String, Type)>,
    pub definitions: Vec<(Rc<RefCell<Trait>>, Evaluation)>,
    id: usize,
}

impl Class {
    pub fn constructor_inputs(
        module_statement: &ModuleStatement,
        context: &SemanticContext,
    ) -> CResult<Vec<(String, Type)>> {
        let mut inputs = vec![];

        let dependencies = &module_statement.class.as_ref().unwrap().dependencies;

        for (name, type_statement) in dependencies.iter() {
            let typ = Type::analyse(type_statement, context, &module_statement.name)?;

            inputs.push((name.clone(), typ));
        }

        Ok(inputs)
    }

    pub fn analyse(module_statement: &ModuleStatement, context: &SemanticContext) -> CResult<Self> {
        let dependencies = Self::constructor_inputs(module_statement, context)?;

        let path = &module_statement.name;

        let mut scope = SemanticScope {
            context,
            path,
            locals: HashMap::new(),
            zelf: Some(interface_type(
                context.interfaces.resolve(path, "")?.borrow().as_ref(),
            )),
        };

        let mut used_interfaces = vec![];

        let mut definitions = vec![];
        for def_statement in module_statement.defs.iter() {
            let trayt = context.traits.resolve(&def_statement.name, path)?;

            used_interfaces.push(trayt.borrow().interface.clone());

            scope.locals = [dependencies.clone(), trayt.borrow().inputs.clone()]
                .concat()
                .into_iter()
                .collect();

            let mut evaluation = Evaluation::analyse(def_statement.expr.clone(), &scope)?;
            coerce_type(&trayt.borrow().output, &mut evaluation, &scope)?;

            definitions.push((trayt, evaluation));
        }

        // TODO: use interface instead of going through definitions and used interfaces
        // Add automatic definitions from other modules.
        for interface in used_interfaces.into_iter() {
            for trayt in interface.borrow().iter() {
                // Skip if the trait has already been defined.
                if definitions.iter().any(|(t, _)| t == trayt) {
                    continue;
                }

                if let Some(eval) = &trayt.borrow().default_definition {
                    definitions.push((Rc::clone(trayt), eval.clone()))
                }
            }
        }

        let class = Class {
            dependencies,
            definitions,
            id: ID.fetch_add(1, Ordering::SeqCst),
        };

        Ok(class)
    }

    pub fn constructor(self) -> Let {
        Let {
            inputs: self.dependencies.clone(),
            output: self.interface(),
            evaluation: Evaluation::ClassConstructor(Rc::new(self)),
        }
    }

    pub fn interface(&self) -> Type {
        let types = self
            .definitions
            .iter()
            .map(|(trayt, _)| Type::Trait(Rc::clone(trayt)))
            .collect::<Vec<_>>();

        combine_types(types)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
