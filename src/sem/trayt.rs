use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Pointer};
use std::rc::Rc;

use crate::ast::module_statement::ModuleStatement;
use crate::ast::trait_statement::TraitStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;

use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::typ::{combine_types, Type};

/// A trait has input types and an output type. It can be defined on classes and structs.
#[derive(Clone)]
pub struct Trait {
    pub full_name: String, // Used for raw operations and for equality checking
    pub interface: Rc<RefCell<Interface>>, // Used to do automatic traits
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
    pub default_definition: Option<Evaluation>,
}

/// An interface is a set of traits.
pub type Interface = Vec<Rc<RefCell<Trait>>>;

pub fn interface_type(interface: &Interface) -> Type {
    let types = interface.iter().cloned().map(Type::Trait).collect();

    combine_types(types)
}

impl Trait {
    pub fn dummy(interface: &Rc<RefCell<Interface>>) -> Self {
        Trait {
            full_name: String::new(),
            interface: Rc::clone(interface),
            inputs: vec![],
            output: Type::Void,
            default_definition: None,
        }
    }

    pub fn analyse(
        statement: &TraitStatement,
        module: &ModuleStatement,
        context: &SemanticContext,
        with_default_definition: bool,
    ) -> CResult<Self> {
        let path = &module.name;

        let mut inputs = vec![];
        for (param_name, type_statement) in statement.parameters.iter() {
            let typ = Type::analyse(type_statement, context, path)?;

            inputs.push((param_name.clone(), typ));
        }

        let full_name = format!("{}\\{}", path, statement.name);

        // Analyse default def, if provided.
        let default_definition = if with_default_definition {
            let def = module
                .defs
                .iter()
                .find(|def| def.name == statement.name || def.name == full_name);

            if let Some(def) = def {
                let scope = SemanticScope {
                    context,
                    path,
                    locals: inputs.iter().cloned().collect(),
                    zelf: Some(interface_type(
                        context.interfaces.resolve(path, "")?.borrow().as_ref(),
                    )),
                };

                match Evaluation::analyse(&def.expr, &scope) {
                    Ok(eval) => Some(eval),
                    // If the evaluation can't be analysed without the context of a struct or class,
                    // then it isn't suitable as a default definition of the trait.
                    Err(_) => None,
                }
            } else {
                None
            }
        } else {
            None
        };

        let trayt = Trait {
            full_name,
            interface: context.interfaces.resolve(path, "")?,
            inputs,
            output: Type::analyse(&statement.output, context, path)?,
            default_definition,
        };

        Ok(trayt)
    }
}

impl PartialEq for Trait {
    fn eq(&self, other: &Self) -> bool {
        self.full_name == other.full_name
    }
}

impl Debug for Trait {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Trayt")
            .field("full_name", &self.full_name)
            .finish()
    }
}