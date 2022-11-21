use crate::ast::module_statement::ModuleStatement;
use crate::ast::struct_statement::StructStatement;
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// A struct has a set of fields which are of raw types, and a set of trait definitions.
pub struct Struct {
    pub fields: Vec<(String, RawType)>,
    pub definitions: Vec<(Rc<RefCell<Trait>>, Evaluation)>,
}

impl Struct {
    pub fn constructor_inputs(statement: &StructStatement) -> Vec<(String, Type)> {
        statement
            .fields
            .iter()
            .map(|(name, typ)| (name.clone(), Type::Raw(*typ)))
            .collect()
    }

    pub fn analyse(module_statement: &ModuleStatement, context: &SemanticContext) -> CResult<Self> {
        let struct_statement = module_statement.strukt.as_ref().unwrap();

        let constructor_inputs = Self::constructor_inputs(struct_statement);

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

            scope.locals = [constructor_inputs.clone(), trayt.borrow().inputs.clone()]
                .concat()
                .into_iter()
                .collect();

            let evaluation = Evaluation::analyse(&def_statement.expr, &scope)?;

            definitions.push((trayt, evaluation));
        }

        let strukt = Struct {
            fields: struct_statement.fields.clone(),
            definitions,
        };

        Ok(strukt)
    }

    pub fn constructor(self) -> Let {
        let inputs = self
            .fields
            .iter()
            .map(|(name, typ)| (name.clone(), Type::Raw(*typ)))
            .collect();

        Let {
            inputs,
            output: self.interface(),
            evaluation: Evaluation::StructConstructor(Rc::new(self)),
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
