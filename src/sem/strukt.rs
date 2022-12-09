use crate::ast::module_statement::ModuleStatement;
use crate::ast::struct_statement::StructStatement;
use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::trayt::{interface_type, Trait};
use crate::sem::typ::{combine_types, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::sem::type_checking::type_contains;

/// A struct has a set of fields which are of raw types, and a set of trait definitions.
#[derive(Debug)]
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
            zelf: Some(interface_type(
                context.interfaces.resolve(path, "")?.borrow().as_ref(),
            )),
        };

        let mut used_interfaces = vec![];

        let mut definitions = vec![];
        for def_statement in module_statement.defs.iter() {
            let trayt = context.traits.resolve(&def_statement.name, path)?;

            used_interfaces.push(trayt.as_ref().borrow().interface.clone());

            scope.locals = [constructor_inputs.clone(), trayt.borrow().inputs.clone()]
                .concat()
                .into_iter()
                .collect();

            // Add friendly fields to locals, by checking all locals that are of the Self type.
            let mut friendly_locals = vec![];
            for (local_name, typ) in scope.locals.iter() {
                if type_contains(typ, &Type::Zelf) {
                    for (field_name, field_type) in &struct_statement.fields {
                        let friendly_field_name = format!("{}.{}", local_name, field_name);

                        friendly_locals.push((friendly_field_name, Type::Raw(*field_type)));
                    }
                }
            }
            for friendly_local in friendly_locals {
                scope.locals.insert(friendly_local.0, friendly_local.1);
            }

            let evaluation = Evaluation::analyse(&def_statement.expr, &scope)?;

            definitions.push((trayt, evaluation));
        }

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

        let strukt = Struct {
            fields: struct_statement.fields.clone(),
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
