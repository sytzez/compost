use crate::ast::trait_statement::TraitStatement;
use crate::error::CResult;
use crate::sem::semantic_analyser::SemanticContext;
use crate::sem::typ::Type;

// A trait has input types and an output type. It can be defined on classes and structs.
#[derive(Eq, PartialEq)]
pub struct Trait {
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
}

impl Trait {
    pub fn analyse(statement: &TraitStatement, context: &SemanticContext) -> CResult<Self> {
        let mut inputs = vec![];
        for (param_name, type_statement) in statement.parameters.iter() {
            let typ = Type::analyse(type_statement, context)?;

            inputs.push((param_name.clone(), typ));
        }

        let output = Type::analyse(&statement.output, context)?;

        let trayt = Trait { inputs, output };

        Ok(trayt)
    }
}