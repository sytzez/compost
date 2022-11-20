use std::cell::RefCell;
use crate::ast::expression::Expression;
use crate::sem::typ::{combine_types, Type};
use crate::sem::lett::Let;
use crate::sem::scope::{path, ReferencePath};
use std::rc::Rc;
use std::string::String;
use crate::ast::class_statement::ClassStatement;
use crate::ast::def_statement::DefStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;

// A class has a set of dependencies of certain types, and a set of trait definitions.
pub struct Class {
    pub dependencies: Vec<(String, Type)>,
    pub definitions: Vec<(Rc<RefCell<Option<Trait>>>, Evaluation)>,
}

impl Class {
    /// Step one of analysis.
    pub fn analyse(statement: &ClassStatement, def_statements: &[DefStatement], context: &SemanticContext) -> CResult<Self> {
        let mut dependencies = vec![];
        for (name, type_statement) in statement.dependencies.iter() {
            let typ = Type::analyse(type_statement, context)?;

            dependencies.push((name.clone(), typ));
        }

        // We process just the traits for now, so the struct will have an accurate interface.
        let mut definitions = vec![];
        for def_statement in def_statements.iter() {
            let trayt = context.traits.resolve(&path(&def_statement.name))?;

            // A dummy evaluation for now...
            let evaluation = Evaluation::Zelf;

            definitions.push((trayt, evaluation));
        }


        let class = Class { dependencies, definitions };

        Ok(class)
    }

    /// Step two of analysis, after all types and lets have been identified. Returns a new class to replace the previous one.
    pub fn analyse_definitions(&self, def_statements: &[DefStatement], context: &SemanticContext) -> CResult<Self> {
        // TODO: create special context with fields and self.
        let mut definitions = vec![];
        for def_statement in def_statements.iter() {
            let trayt = context.traits.resolve(&path(&def_statement.name))?;

            let evaluation = Evaluation::analyse(&def_statement.expr, context)?;

            definitions.push((trayt, evaluation));
        }

        let class = Class {
            dependencies: self.dependencies.clone(),
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
