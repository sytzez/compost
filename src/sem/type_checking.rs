use crate::error::{error, CResult, ErrorMessage};
use crate::sem::evaluation::Evaluation;
use crate::sem::semantic_analyser::SemanticScope;
use crate::sem::typ::Type;

/// Checks if a set of inputed evaluations satisfies a set of expected types.
pub fn check_types(
    types: &[(String, Type)],
    inputs: &[(String, Evaluation)],
    scope: &SemanticScope,
) -> CResult<()> {
    for (name, typ) in types {
        if let Some((_, input)) = inputs.iter().find(|(other_name, _)| other_name == name) {
            if !input.typ(scope)?.fits(typ) {
                return error(ErrorMessage::TypeMismatch(name.clone()));
            }
        } else {
            return error(ErrorMessage::MissingInput(name.clone()));
        }
    }
    Ok(())
}
