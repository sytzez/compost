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
use std::rc::Rc;

/// All available symbols in a program
pub struct SemanticContext {
    pub traits: Table<RefCell<Trait>>,
    pub lets: Table<RefCell<Let>>,
    pub interfaces: Table<RefCell<Interface>>,
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

    // Populate trait and interface identifiers.
    for module in ast.mods.iter() {
        let dummy_interface = context
            .interfaces
            .declare(&module.name, RefCell::new(vec![]))?;

        for trait_statement in module.traits.iter() {
            let name = format!("{}\\{}", module.name, trait_statement.name);

            context
                .traits
                .declare(&name, RefCell::new(Trait::dummy(&name, &dummy_interface)))?;
        }

        // Each module has an eponymous trait, which has the module interface as output type.
        context.traits.declare(
            &module.name,
            RefCell::new(Trait::dummy(&module.name, &dummy_interface)),
        )?;
    }

    // Fill module interfaces, made up of the module's own traits and def traits from other modules.
    // By this point, all trait identifiers have been populated.
    for module in ast.mods.iter() {
        let mut interface = vec![];

        // The module's own traits.
        for trait_statement in module.traits.iter() {
            let trayt = context
                .traits
                .resolve(&trait_statement.name, &module.name)?;

            interface.push(trayt);
        }

        // Traits added on from other modules through defs.
        for def in module.defs.iter() {
            let trayt = context.traits.resolve(&def.name, &module.name)?;

            interface.push(trayt);
        }

        let output = interface_type(&interface);

        context
            .interfaces
            .resolve(&module.name, "")?
            .replace(interface);

        let eponymous_trait = Trait {
            full_name: module.name.clone(),
            interface: context.interfaces.resolve(&module.name, "")?,
            inputs: vec![],
            output,
            default_definition: None,
        };

        context
            .traits
            .resolve(&module.name, "")?
            .replace(eponymous_trait);
    }

    // Add automatic definitions to interfaces. Repeat until stable.
    loop {
        let mut added_num_of_traits: usize = 0;

        for module in ast.mods.iter() {
            let own_interface = context.interfaces.resolve(&module.name, "")?;

            let mut related_interfaces = vec![];

            for def in module.defs.iter() {
                let trayt = context.traits.resolve(&def.name, &module.name)?;

                related_interfaces.push(Rc::clone(&trayt.borrow().interface));
            }

            for related_interface in related_interfaces.iter() {
                for trayt in related_interface.borrow().iter() {
                    if own_interface.borrow().iter().any(|t| t == trayt) {
                        continue;
                    }

                    own_interface.replace_with(|old| {
                        old.push(Rc::clone(trayt));
                        old.clone()
                    });

                    added_num_of_traits += 1
                }
            }
        }

        if added_num_of_traits == 0 {
            break;
        }
    }

    // ==========================================================================================
    // STEP 2: Analyse trait, let and def *input and output types*.
    // By this point, all trait and interface types have been populated, making it possible to
    // analyse any type.
    // ==========================================================================================

    // Populate global let identifiers and types.
    for let_statement in ast.lets.iter() {
        let lett = Let::analyse_just_types(let_statement, &context, "")?;

        context
            .lets
            .declare(&let_statement.name, RefCell::new(lett))?;
    }

    for module in ast.mods.iter() {
        // Analyse trait input and output types, and default definitions.
        for trait_statement in module.traits.iter() {
            let trayt = Trait::analyse(trait_statement, module, &context, false)?;

            context
                .traits
                .resolve(&trait_statement.name, &module.name)?
                .replace(trayt);
        }

        // Populate module let identifiers.
        for let_statement in module.lets.iter() {
            let name = format!("{}\\{}", module.name, let_statement.name);

            let lett = Let::analyse_just_types(let_statement, &context, &module.name)?;

            context.lets.declare(&name, RefCell::new(lett))?;
        }

        // Populate struct and class constructor and def identifiers.
        if let Some(struct_statement) = &module.strukt {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Struct::constructor_inputs(struct_statement),
                output: interface_type(
                    context
                        .interfaces
                        .resolve(&module.name, "")?
                        .borrow()
                        .as_ref(),
                ),
                evaluation: Evaluation::Zelf,
            };

            context
                .lets
                .declare(&module.name, RefCell::new(constructor))?;
        } else if module.class.is_some() {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Class::constructor_inputs(module, &context)?,
                output: interface_type(
                    context
                        .interfaces
                        .resolve(&module.name, "")?
                        .borrow()
                        .as_ref(),
                ),
                evaluation: Evaluation::Zelf,
            };

            context
                .lets
                .declare(&module.name, RefCell::new(constructor))?;
        }
    }

    // ==========================================================================================
    // STEP 3: Analyse let and def *expressions*.
    // By this point, all trait types and let types have been established, making it possible to
    // analyse any expression.
    // ==========================================================================================

    // Analyse global let expressions.
    for let_statement in ast.lets.iter() {
        let lett = Let::analyse(let_statement, &context, "")?;

        context.lets.resolve(&let_statement.name, "")?.replace(lett);
    }

    for module in ast.mods.iter() {
        // Analyse module let expressions.
        for let_statement in module.lets.iter() {
            let lett = Let::analyse(let_statement, &context, &module.name)?;

            context
                .lets
                .resolve(&let_statement.name, &module.name)?
                .replace(lett);
        }

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
