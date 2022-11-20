use std::cell::RefCell;
use crate::sem::typ::{combine_types, Type};
use crate::sem::lett::Let;
use std::rc::Rc;
use crate::ast::def_statement::DefStatement;
use crate::ast::struct_statement::StructStatement;
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::scope::path;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::trayt::Trait;

/// A struct has a set of fields which are of raw types, and a set of trait definitions.
pub struct Struct {
    pub fields: Vec<(String, RawType)>,
    pub definitions: Vec<(Rc<RefCell<Option<Trait>>>, Evaluation)>,
}

impl Struct {
    /// Step one of analysis.
    pub fn analyse(statement: &StructStatement, def_statements: &[DefStatement], context: &SemanticContext) -> CResult<Self> {
        // We process just the traits for now, so the struct will have an accurate interface.
        let mut definitions = vec![];
        for def_statement in def_statements.iter() {
            let trayt = context.traits.resolve(&path(&def_statement.name))?;

            // A dummy evaluation for now...
            let evaluation = Evaluation::Zelf;

            definitions.push((trayt, evaluation));
        }

        let strukt = Struct {
            fields: statement.fields.clone(),
            definitions,
        };

        Ok(strukt)
    }

    /// Step two of analysis, after all types and lets have been identified. Returns a new struct to replace the previous one.
    pub fn analyse_definitions(&self, def_statements: &[DefStatement], context: &SemanticContext) -> CResult<Self> {
        // TODO: create special context with fields and self.
        let mut definitions = vec![];
        for def_statement in def_statements.iter() {
            let trayt = context.traits.resolve(&path(&def_statement.name))?;

            let evaluation = Evaluation::analyse(&def_statement.expr, context)?;

            definitions.push((trayt, evaluation));
        }

        let strukt = Struct {
            fields: self.fields.clone(),
            definitions,
        };

        Ok(strukt)
    }

    pub fn constructor(self: &Rc<Self>) -> Let {
        let inputs = self
            .fields
            .iter()
            .map(|(name, typ)| (name.clone(), Type::Raw(*typ)))
            .collect();

        Let {
            inputs,
            output: self.interface(),
            evaluation: Evaluation::StructConstructor(Rc::clone(self)),
        }
    }

    pub fn interface(&self) -> Type {
        let types = self
            .definitions
            .iter()
            .map(|(trayt, _)| Type::Trait(Rc::clone(trayt)))
            .collect();

        combine_types(types)
    }
}
