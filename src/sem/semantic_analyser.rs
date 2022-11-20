use std::cell::RefCell;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::ast::type_statement::TypeStatement;
use crate::sem::typ::Type;
use crate::error::CResult;
use crate::sem::lett::Let;
use crate::sem::scope::{path, Scope, Table};
use crate::sem::trayt::Trait;


pub fn analyse_ast(ast: AbstractSyntaxTree) -> CResult<Scope> {
    let mut scope = Scope::new();

    // Populate traits identifiers.
    let mut traits: Table<RefCell<Option<Trait>>> = Table::new();
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));
            traits.add(path, RefCell::new(None));
        }
    }

    // Connect trait input and output types.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));

            let inputs = trait_statement.parameters
                .iter()
                .map(|(param_name, type_statement)| {
                    let typ = analyse_type(type_statement, &traits);

                    (param_name.clone(), typ)
                })
                .collect();

            let output = analyse_type(&trait_statement.output, &traits);

            let trayt = Trait { inputs, output, };

            traits.resolve(&path).replace(Some(trayt));
        }
    }

    // Populate let identifiers from lets + struct and class constructors

    // Add let input types.

    // Parse let expressions, check types.
    let mut lets = Table::new();
    for let_statement in ast.lets.into_iter() {
        let path = path(&let_statement.name);
        let lett = Let::from(let_statement);
        lets.add(path, lett)
    }

    Ok(scope)
}

fn analyse_type(statement: &TypeStatement, traits: &Table<RefCell<Option<Trait>>>) -> Type {
    match statement {
        TypeStatement::Name(path_name) => {
            // TODO: check if it's a module name -> use module interface
            let trayt = traits.resolve(&path(path_name));
            Type::Trait(trayt)
        }
        TypeStatement::And(left, right) => {
            Type::And(
                Box::new(analyse_type(left, traits)),
                Box::new(analyse_type(right,traits)),
            )
        }
        TypeStatement::Or(left, right) => {
            Type::Or(
                Box::new(analyse_type(left, traits)),
                Box::new(analyse_type(right,traits)),
            )
        }
        TypeStatement::Zelf => Type::Zelf,
        TypeStatement::Void => Type::Void,
    }
}
