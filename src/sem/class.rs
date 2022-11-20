use std::cell::RefCell;

use crate::ast::class_statement::ClassStatement;
use crate::ast::def_statement::DefStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::scope::path;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use std::rc::Rc;
use std::string::String;

// A class has a set of dependencies of certain types, and a set of trait definitions.
pub struct Class {
    pub dependencies: Vec<(String, Type)>,
    pub definitions: Vec<(Rc<RefCell<Option<Trait>>>, Evaluation)>,
}

impl Class {
    pub fn constructor_inputs(
        statement: &ClassStatement,
        context: &SemanticContext,
    ) -> CResult<Vec<(String, Type)>> {
        let mut inputs = vec![];

        for (name, type_statement) in statement.dependencies.iter() {
            let typ = Type::analyse(type_statement, context)?;

            inputs.push((name.clone(), typ));
        }

        Ok(inputs)
    }

    pub fn analyse(
        statement: &ClassStatement,
        def_statements: &[DefStatement],
        context: &SemanticContext,
    ) -> CResult<Self> {
        let dependencies = Self::constructor_inputs(statement, context)?;

        // TODO: create special context with fields and self.
        let mut definitions = vec![];
        for def_statement in def_statements.iter() {
            let trayt = context.traits.resolve(&path(&def_statement.name))?;

            let evaluation = Evaluation::analyse(&def_statement.expr, context)?;

            definitions.push((trayt, evaluation));
        }

        let class = Class {
            dependencies,
            definitions,
        };

        Ok(class)
    }

    pub fn constructor(self: &Rc<Self>) -> Let {
        Let {
            inputs: self.dependencies.clone(),
            output: self.interface(),
            evaluation: Evaluation::ClassConstructor(Rc::clone(self)),
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
