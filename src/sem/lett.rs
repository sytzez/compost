use crate::sem::typ::Type;

use crate::ast::let_statement::LetStatement;
use crate::error::CResult;
use crate::sem::evaluation::Evaluation;
use crate::sem::semantic_analyser::SemanticContext;

// A 'let' defines a constant instance or a function.
pub struct Let {
    pub inputs: Vec<(String, Type)>,
    pub output: Type,
    pub evaluation: Evaluation,
}

// TODO: this should go somewhere in the runtime module.
// impl Let {
//     pub fn resolve(&self, inputs: HashMap<String, Rc<Instance>>, scope: &Scope) -> Rc<Instance> {
//         let local_scope = scope.local_scope(None, inputs);
//
//         self.expression.resolve(&local_scope)
//     }
// }

impl Let {
    /// An earlier stage analysis that analyses only the inputs and output.
    pub fn analyse_just_types(
        statement: &LetStatement,
        context: &SemanticContext,
    ) -> CResult<Self> {
        let mut inputs = vec![];
        for (param_name, type_statement) in statement.parameters.iter() {
            let typ = Type::analyse(type_statement, context)?;

            inputs.push((param_name.clone(), typ));
        }

        let output = Type::analyse(&statement.output, context)?;

        let lett = Let {
            inputs,
            output,
            evaluation: Evaluation::Zelf,
        };

        Ok(lett)
    }

    /// The final analysis.
    pub fn analyse(statement: &LetStatement, context: &SemanticContext) -> CResult<Self> {
        let lett = Self::analyse_just_types(statement, context)?;

        let evaluation = Evaluation::analyse(&statement.expr, context)?;

        // TODO: check if output type fits evaluation output type
        let lett = Let {
            inputs: lett.inputs,
            output: lett.output,
            evaluation,
        };

        Ok(lett)
    }
}
