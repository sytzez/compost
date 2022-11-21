use std::cell::RefCell;
use std::collections::HashMap;

use crate::ast::module_statement::ModuleStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use std::rc::Rc;
use std::string::String;

// A class has a set of dependencies of certain types, and a set of trait definitions.
pub struct Class {
    pub dependencies: Vec<(String, Type)>,
    pub definitions: Vec<(Rc<RefCell<Trait>>, Evaluation)>,
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
            zelf: Some(context.interfaces.resolve(path, "")?),
        };

        let mut definitions = vec![];
        for def_statement in module_statement.defs.iter() {
            let trayt = context.traits.resolve(&def_statement.name, path)?;

            scope.locals = [dependencies.clone(), trayt.borrow().inputs.clone()]
                .concat()
                .into_iter()
                .collect();

            let evaluation = Evaluation::analyse(&def_statement.expr, &scope)?;

            definitions.push((trayt, evaluation));
        }

        let class = Class {
            dependencies,
            definitions,
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
