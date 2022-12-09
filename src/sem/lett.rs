use crate::sem::typ::Type;

use crate::ast::let_statement::LetStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::semantic_analyser::{SemanticContext, SemanticScope};
use crate::sem::type_coercion::coerce_type;

// A 'let' defines a constant instance or a function.
#[derive(Debug)]
pub struct Let {
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
    pub evaluation: Evaluation,
}

impl Let {
    /// An earlier stage analysis that analyses only the inputs and output.
    pub fn analyse_just_types(
        statement: &LetStatement,
        context: &SemanticContext,
        path: &str,
    ) -> CResult<Self> {
        let mut inputs = vec![];
        for (param_name, type_statement) in statement.parameters.iter() {
            let typ = Type::analyse(type_statement, context, path)?;

            inputs.push((param_name.clone(), typ));
        }

        let output = Type::analyse(&statement.output, context, path)?;

        let lett = Let {
            inputs,
            output,
            evaluation: Evaluation::Zelf,
        };

        Ok(lett)
    }

    /// The final analysis.
    pub fn analyse(
        statement: &LetStatement,
        context: &SemanticContext,
        path: &str,
    ) -> CResult<Self> {
        let lett = Self::analyse_just_types(statement, context, path)?;

        let scope = SemanticScope {
            context,
            path,
            locals: lett.inputs.iter().cloned().collect(),
            zelf: None,
        };

        let mut evaluation = Evaluation::analyse(&statement.expr, &scope)?;
        coerce_type(&lett.output, &mut evaluation, &statement.name, &scope)?;

        // TODO: check if output type fits evaluation output type
        let lett = Let {
            inputs: lett.inputs,
            output: lett.output,
            evaluation,
        };

        Ok(lett)
    }
}
