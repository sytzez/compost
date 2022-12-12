use crate::ast::type_statement::RawType;
use crate::error::CResult;
use crate::sem::evaluation::{Evaluation, LetEvaluation};
use crate::sem::semantic_analyser::SemanticScope;
use crate::sem::typ::Type;
use crate::sem::type_checking::check_type_fits;

/// Coerces raw literals into stdlib structs where possible.
pub fn coerce_types(
    types: &[(String, Type)],
    inputs: &mut Vec<(String, Evaluation)>,
    scope: &SemanticScope,
) -> CResult<()> {
    for (name, eval) in inputs {
        let typ = types.iter().find(|(type_name, _)| type_name == name);

        if let Some((_, typ)) = typ {
            coerce_type(typ, eval, scope)?;
        }
    }

    Ok(())
}

/// Coerces raw literals into stdlib structs where possible.
pub fn coerce_type(
    typ: &Type,
    eval: &mut Evaluation,
    scope: &SemanticScope,
) -> CResult<()> {
    let eval_type = eval.typ(scope)?;

    if check_type_fits(&eval_type, typ).is_ok() {
        // If the type already fits, don't do anything.
        return Ok(());
    } else if let Type::Raw(raw_type) = eval_type {
        // If the type is raw, turn int into a struct constructor.
        let new_eval = coerce_raw_to_struct(&raw_type, eval, scope)?;
        let _ = std::mem::replace(eval, new_eval);
    }

    Ok(())
}

fn coerce_raw_to_struct(
    raw_type: &RawType,
    eval: &Evaluation,
    scope: &SemanticScope,
) -> CResult<Evaluation> {
    let lett_name = match raw_type {
        RawType::Int => "Int",
        RawType::String => "String",
    };

    let lett = scope.context.lets.resolve(lett_name, scope.path)?;

    let inputs = [("value".to_string(), eval.clone())].into();

    Ok(Evaluation::Let(LetEvaluation { lett, inputs }))
}
