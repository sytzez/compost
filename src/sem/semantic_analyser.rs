use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::error::CResult;
use crate::sem::class::Class;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::strukt::Struct;
use crate::sem::table::Table;
use crate::sem::trayt::{interface_type, Interface, Trait};
use crate::sem::typ::Type;
use std::cell::RefCell;
use std::collections::HashMap;

/// All available symbols in a program
pub struct SemanticContext {
    pub traits: Table<RefCell<Trait>>,
    pub lets: Table<RefCell<Let>>,
    pub interfaces: Table<Interface>,
}

impl SemanticContext {
    pub fn new() -> Self {
        SemanticContext {
            lets: Table::new("Let"),
            traits: Table::new("Trait"),
            interfaces: Table::new("Interface"),
        }
    }
}

// All available symbols inside a scope
pub struct SemanticScope<'a> {
    pub context: &'a SemanticContext,
    pub path: &'a str,
    pub locals: HashMap<String, Type>,
    pub zelf: Option<Type>,
}

/// Analyses the semantics of a complete AST, and returns the global semantic context.
pub fn analyse_ast(ast: AbstractSyntaxTree) -> CResult<SemanticContext> {
    let mut context = SemanticContext::new();

    // ==========================================================================================
    // STEP 1: Populate trait and interface identifiers.
    // ==========================================================================================

    // Populate trait identifiers.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let name = format!("{}\\{}", module.name, trait_statement.name);

            context
                .traits
                .declare(&name, RefCell::new(Trait::dummy()))?;
        }

        // Each module has an eponymous trait, which has the module interface as output type.
        context
            .traits
            .declare(&module.name, RefCell::new(Trait::dummy()))?;
    }

    // Populate module interfaces, made up of the module's own traits and def traits from other modules.
    // By this point, all trait identifiers have been populated.
    for module in ast.mods.iter() {
        let mut interface = vec![];

        // The modules own traits.
        for trait_statement in module.traits.iter() {
            let trayt = context
                .traits
                .resolve(&trait_statement.name, &module.name)?;

            interface.push(trayt);
        }

        // Traits added on from other modules through defs.
        for def_statement in module.defs.iter() {
            let trayt = context.traits.resolve(&def_statement.name, &module.name)?;

            interface.push(trayt);
        }

        let interface = context.interfaces.declare(&module.name, interface)?;

        let output = interface_type(&interface);

        let eponymous_trait = Trait {
            full_name: module.name.clone(),
            interface,
            inputs: vec![],
            output,
            default_definition: None,
        };

        context
            .traits
            .resolve(&module.name, "")?
            .replace(eponymous_trait);
    }

    // ==========================================================================================
    // STEP 2: Analyse trait, let and def input and output types
    // By this point, all trait and interface types have been populated, making it possible to
    // analyse any type.
    // ==========================================================================================

    // Analyse trait input and output types.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let trayt = Trait::analyse(trait_statement, module, &context, false)?;

            context
                .traits
                .resolve(&trait_statement.name, &module.name)?
                .replace(trayt);
        }
    }

    // Populate global let identifiers and types.
    for let_statement in ast.lets.iter() {
        let lett = Let::analyse_just_types(let_statement, &context, "")?;

        context
            .lets
            .declare(&let_statement.name, RefCell::new(lett))?;
    }

    // Populate module let identifiers.
    for module in ast.mods.iter() {
        for let_statement in module.lets.iter() {
            let name = format!("{}\\{}", module.name, let_statement.name);

            let lett = Let::analyse_just_types(let_statement, &context, &module.name)?;

            context.lets.declare(&name, RefCell::new(lett))?;
        }
    }

    // Populate struct and class constructor and def identifiers.
    for module in ast.mods.iter() {
        if let Some(struct_statement) = &module.strukt {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Struct::constructor_inputs(struct_statement),
                output: interface_type(context.interfaces.resolve(&module.name, "")?.as_ref()),
                evaluation: Evaluation::Zelf,
            };

            context
                .lets
                .declare(&module.name, RefCell::new(constructor))?;
        } else if module.class.is_some() {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Class::constructor_inputs(module, &context)?,
                output: interface_type(context.interfaces.resolve(&module.name, "")?.as_ref()),
                evaluation: Evaluation::Zelf,
            };

            context
                .lets
                .declare(&module.name, RefCell::new(constructor))?;
        }
    }

    // ==========================================================================================
    // STEP 3: Analyse let and def expressions.
    // By this point, all trait types and let types have been established, making it possible to
    // analyse any expression.
    // ==========================================================================================

    // Analyse global let expressions.
    for let_statement in ast.lets.iter() {
        let lett = Let::analyse(let_statement, &context, "")?;

        context.lets.resolve(&let_statement.name, "")?.replace(lett);
    }

    // Analyse module let expressions.
    for module in ast.mods.iter() {
        for let_statement in module.lets.iter() {
            let lett = Let::analyse(let_statement, &context, &module.name)?;

            context
                .lets
                .resolve(&let_statement.name, &module.name)?
                .replace(lett);
        }
    }

    for module in ast.mods.iter() {
        // Re-analyse traits with default definitions.
        for trait_statement in module.traits.iter() {
            let trayt = Trait::analyse(trait_statement, module, &context, true)?;

            context
                .traits
                .resolve(&trait_statement.name, &module.name)?
                .replace(trayt);
        }

        // Analyse struct and class constructor and def expressions.
        if module.strukt.is_some() {
            let strukt = Struct::analyse(module, &context)?;

            context
                .lets
                .resolve(&module.name, "")?
                .replace(strukt.constructor());
        } else if module.class.is_some() {
            let class = Class::analyse(module, &context)?;

            context
                .lets
                .resolve(&module.name, "")?
                .replace(class.constructor());
        }
    }

    Ok(context)
}
