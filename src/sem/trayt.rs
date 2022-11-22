use std::cell::RefCell;
use std::rc::Rc;
use crate::ast::def_statement::DefStatement;
use crate::ast::expression::Expression;
use crate::ast::module_statement::ModuleStatement;
use crate::ast::trait_statement::TraitStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::typ::{combine_types, Type};

/// A trait has input types and an output type. It can be defined on classes and structs.
#[derive(Clone)]
pub struct Trait {
    pub full_name: String, // Used for raw operations and for equality checking
    pub interface: Rc<Interface>, // Used to do automatic traits
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
    pub default_expr: Option<Expression>,
}

/// An interface is a set of traits.
pub type Interface = Vec<Rc<RefCell<Trait>>>;

pub fn interface_type(interface: &Interface) -> Type {
    let types = interface.iter().cloned().map(Type::Trait).collect();

    combine_types(types)
}

impl Trait {
    pub fn dummy() -> Self {
        Trait {
            full_name: String::new(),
            interface: Rc::new(vec![]),
            inputs: vec![],
            output: Type::Void,
            default_expr: None,
        }
    }

    pub fn analyse(
        statement: &TraitStatement,
        module: &ModuleStatement,
        context: &SemanticContext,
    ) -> CResult<Self> {
        let path = &module.name;

        let mut inputs = vec![];
        for (param_name, type_statement) in statement.parameters.iter() {
            let typ = Type::analyse(type_statement, context, path)?;

            inputs.push((param_name.clone(), typ));
        }

        let full_name = format!("{}\\{}", path, statement.name);

        let default_expr = module.defs
            .iter()
            .find(|def| def.name == statement.name || def.name == full_name)
            .map(|def| def.expr.clone());

        let trayt = Trait {
            full_name,
            interface: context.interfaces.resolve(path, "")?,
            inputs,
            output: Type::analyse(&statement.output, context, path)?,
            default_expr,
        };

        Ok(trayt)
    }
}

impl PartialEq for Trait {
    fn eq(&self, other: &Self) -> bool {
        self.full_name == other.full_name
    }
}

