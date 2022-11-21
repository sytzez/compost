use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::error::CResult;
use crate::sem::class::Class;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::strukt::Struct;
use crate::sem::table::Table;
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SemanticContext {
    pub traits: Table<RefCell<Trait>>,
    pub lets: Table<RefCell<Let>>,
    pub interfaces: Table<Type>,
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

pub struct SemanticScope<'a> {
    pub context: &'a SemanticContext,
    pub path: &'a str,
    pub locals: HashMap<String, Type>,
    pub zelf: Option<Rc<Type>>,
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
            let name = &format!("{}\\{}", module.name, trait_statement.name);

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
        let mut types_for_module = vec![];

        // The modules own traits.
        for trait_statement in module.traits.iter() {
            let trayt = context
                .traits
                .resolve(&trait_statement.name, &module.name)?;

            types_for_module.push(Type::Trait(trayt));
        }

        // Traits added on from other modules through defs.
        for def_statement in module.defs.iter() {
            let trayt = context.traits.resolve(&def_statement.name, &module.name)?;

            types_for_module.push(Type::Trait(trayt));
        }

        let interface = combine_types(types_for_module);

        context
            .interfaces
            .declare(&module.name, interface.clone())?;

        let eponymous_trait = Trait {
            inputs: vec![],
            output: interface,
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
            let trayt = Trait::analyse(trait_statement, &context, &module.name)?;

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
                output: context
                    .interfaces
                    .resolve(&module.name, "")?
                    .as_ref()
                    .clone(),
                evaluation: Evaluation::Zelf,
            };

            context
                .lets
                .declare(&module.name, RefCell::new(constructor))?;
        } else if module.class.is_some() {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Class::constructor_inputs(module, &context)?,
                output: context
                    .interfaces
                    .resolve(&module.name, "")?
                    .as_ref()
                    .clone(),
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

    // Analyse struct and class constructor and def expressions.
    for module in ast.mods.iter() {
        if module.strukt.is_some() {
            let strukt = Struct::analyse(&module, &context)?;

            context
                .lets
                .resolve(&module.name, "")?
                .replace(strukt.constructor());
        } else if module.class.is_some() {
            let class = Class::analyse(&module, &context)?;

            context
                .lets
                .resolve(&module.name, "")?
                .replace(class.constructor());
        }
    }

    Ok(context)
}
