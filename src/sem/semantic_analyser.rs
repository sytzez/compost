use std::cell::RefCell;
use std::collections::HashMap;
use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::sem::typ::Type;
use crate::error::CResult;
use crate::sem::lett::Let;
use crate::sem::scope::{path, Table};
use crate::sem::trayt::Trait;

pub struct SemanticContext {
    pub traits: Table<RefCell<Option<Trait>>>,
    pub lets: Table<RefCell<Option<Let>>>,
    pub locals: HashMap<String, Type>,
    pub zelf: Option<Type>,
}

pub fn analyse_ast(ast: AbstractSyntaxTree) -> CResult<SemanticContext> {
    let mut context = SemanticContext {
        zelf: None,
        lets: Table::new(),
        traits: Table::new(),
        locals: HashMap::new(),
    };

    // Populate traits identifiers.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));

            context.traits.add(path, RefCell::new(None));
        }
    }

    // Connect trait input and output types.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));

            let trayt = Trait::analyse(trait_statement, &context)?;

            context.traits.resolve(&path)?.replace(Some(trayt));
        }
    }

    // TODO: add module interface types

    // Populate let identifiers from lets
    for module in ast.mods.iter() {
        for let_statement in module.lets.iter() {
            let path = path(&format!("{}\\{}", module.name, let_statement.name));

            let lett = Let::analyse(let_statement, &context)?;

            context.lets.resolve(&path)?.replace(Some(lett));
        }
    }

    // Add struct contructor lets

    // Add class constructors lets

    // Parse global let expressions, check types.
    for let_statement in ast.lets.into_iter() {
        let _path = path(&let_statement.name);


    }

    // Parse struct let expressions, check types.


    Ok(context)
}
