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
        let input = inputs.iter().find(|(input_name, _)| input_name == name);

        if let Some((_, input)) = input {
            if check_type_fits(&input.typ(scope)?, typ).is_err() {
                return error(ErrorMessage::TypeMismatch(
                    name.clone(),
                    typ.clone(),
                    input.typ(scope)?,
                ));
            }
        } else {
            return error(ErrorMessage::MissingInput(name.clone()));
        }
    }
    Ok(())
}

/// Checks whether the given type is suitable to be used where the expected type is required.
pub fn check_type_fits(given: &Type, expected: &Type) -> Result<(), ()> {
    match expected {
        Type::Void => Ok(()),
        Type::Raw(_) | Type::Zelf | Type::Trait(_) => {
            if type_contains(given, expected) {
                Ok(())
            } else {
                Err(())
            }
        }
        Type::Or(a, b) => match check_type_fits(given, a) {
            Ok(()) => Ok(()),
            Err(_) => check_type_fits(given, b),
        },
        Type::And(a, b) => {
            check_type_fits(given, a)?;
            check_type_fits(given, b)
        }
    }
}

/// Checks whether the given types contains a certain other type within an & chain.
pub fn type_contains(given: &Type, contained: &Type) -> bool {
    if given == contained || contained == &Type::Void {
        true
    } else {
        match given {
            Type::Trait(expected_trait) => {
                if let Type::Trait(contained_trait) = contained {
                    expected_trait.borrow().full_name == contained_trait.borrow().full_name
                } else {
                    false
                }
            }
            Type::And(a, b) => type_contains(a, contained) || type_contains(b, contained),
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::sem::trayt::Trait;
    use crate::sem::typ::Type;
    use crate::sem::type_checking::check_type_fits;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_fits() {
        let interface = Rc::new(RefCell::new(vec![]));
        let trait_a = Rc::new(RefCell::new(Trait::dummy("Trait A", &interface)));
        let trait_b = Rc::new(RefCell::new(Trait::dummy("Trait B", &interface)));

        let a = Type::Trait(trait_a.clone());
        let b = Type::Trait(trait_b.clone());
        let a_and_b = Type::And(
            Box::new(Type::Trait(trait_a.clone())),
            Box::new(Type::Trait(trait_b.clone())),
        );
        let b_and_a = Type::And(
            Box::new(Type::Trait(trait_b.clone())),
            Box::new(Type::Trait(trait_a.clone())),
        );
        let a_or_b = Type::Or(
            Box::new(Type::Trait(trait_a.clone())),
            Box::new(Type::Trait(trait_b.clone())),
        );

        assert_eq!(check_type_fits(&a, &a), Ok(()), "Trait A fits Trait A",);

        assert_eq!(
            check_type_fits(&a, &b),
            Err(()),
            "Trait A doesn't fit Trait B",
        );

        assert_eq!(
            check_type_fits(&a, &a_or_b),
            Ok(()),
            "Trait A fits Trait A or Trait B",
        );

        assert_eq!(
            check_type_fits(&a, &a_and_b),
            Err(()),
            "Trait A doesn't fit Trait A and Trait B",
        );

        assert_eq!(
            check_type_fits(&a_and_b, &a),
            Ok(()),
            "Trait A and Trait B fits Trait A",
        );

        assert_eq!(
            check_type_fits(&a_and_b, &b),
            Ok(()),
            "Trait A and Trait B fits Trait B",
        );

        assert_eq!(
            check_type_fits(&a_and_b, &b_and_a),
            Ok(()),
            "Trait A and Trait B fits Trait B and Trait A",
        );

        assert_eq!(
            check_type_fits(&a_and_b, &a_or_b),
            Ok(()),
            "Trait A and Trait B fits Trait A or Trait B",
        );

        assert_eq!(
            check_type_fits(&a_or_b, &a_and_b),
            Err(()),
            "Trait A or Trait B doesn't fit Trait A and Trait B",
        );
    }
}
