use crate::ast::abstract_syntax_tree::AbstractSyntaxTree;
use crate::error::CResult;
use crate::sem::class::Class;
use crate::sem::evaluation::Evaluation;
use crate::sem::lett::Let;
use crate::sem::scope::{path, Table};
use crate::sem::strukt::Struct;
use crate::sem::trayt::Trait;
use crate::sem::typ::{combine_types, Type};
use std::cell::RefCell;
use std::collections::HashMap;

pub struct SemanticContext {
    pub traits: Table<RefCell<Trait>>,
    pub lets: Table<RefCell<Let>>,
    pub interfaces: Table<Type>,
    pub locals: HashMap<String, Type>,
    pub zelf: Option<Type>,
}

impl SemanticContext {
    pub fn new() -> Self {
        SemanticContext {
            zelf: None,
            lets: Table::new(),
            traits: Table::new(),
            interfaces: Table::new(),
            locals: HashMap::new(),
        }
    }
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
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));

            context.traits.add(path.clone(), RefCell::new(Trait::dummy()));
        }

        // Each module has an eponymous trait, which has the module interface as output type.
        context.traits.add(path(&module.name), RefCell::new(Trait::dummy()));
    }

    // Populate module interfaces, made up of the module's own traits and def traits from other modules.
    // By this point, all trait identifiers have been populated.
    for module in ast.mods.iter() {
        let mut types_for_module = vec![];

        // The modules own traits.
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));
            let trayt = context.traits.resolve(&path)?;

            types_for_module.push(Type::Trait(trayt));
        }

        // Traits added on from other modules through defs.
        for def_statement in module.defs.iter() {
            let path = path(&def_statement.name);
            let trayt = context.traits.resolve(&path)?;

            types_for_module.push(Type::Trait(trayt));
        }

        let path = path(&module.name);
        let interface = combine_types(types_for_module);

        context.interfaces.add(path.clone(), interface.clone());

        let eponymous_trait = Trait {
            inputs: vec![],
            output: interface,
        };
        context.traits.resolve(&path)?.replace(eponymous_trait);
    }

    // ==========================================================================================
    // STEP 2: Analyse trait, let and def input and output types
    // By this point, all trait and interface types have been populated, making it possible to
    // analyse any type.
    // ==========================================================================================

    // Analyse trait input and output types.
    for module in ast.mods.iter() {
        for trait_statement in module.traits.iter() {
            let path = path(&format!("{}\\{}", module.name, trait_statement.name));

            let trayt = Trait::analyse(trait_statement, &context)?;

            context.traits.resolve(&path)?.replace(trayt);
        }
    }

    // Populate global let identifiers and types.
    for let_statement in ast.lets.iter() {
        let path = path(&let_statement.name);

        let lett = Let::analyse_just_types(let_statement, &context)?;

        context.lets.add(path, RefCell::new(lett));
    }

    // Populate module let identifiers.
    for module in ast.mods.iter() {
        for let_statement in module.lets.iter() {
            let path = path(&format!("{}\\{}", module.name, let_statement.name));

            let lett = Let::analyse_just_types(let_statement, &context)?;

            context.lets.add(path, RefCell::new(lett));
        }
    }

    // Populate struct and class constructor and def identifiers.
    for module in ast.mods.iter() {
        let path = path(&module.name);

        if let Some(struct_statement) = &module.strukt {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Struct::constructor_inputs(struct_statement),
                output: context.interfaces.resolve(&path)?.as_ref().clone(),
                evaluation: Evaluation::Zelf,
            };

            context.lets.add(path, RefCell::new(constructor));
        } else if let Some(class_statement) = &module.class {
            // Just the inputs and output of the constructor.
            let constructor = Let {
                inputs: Class::constructor_inputs(class_statement, &context)?,
                output: context.interfaces.resolve(&path)?.as_ref().clone(),
                evaluation: Evaluation::Zelf,
            };

            context.lets.add(path, RefCell::new(constructor));
        }
    }

    // ==========================================================================================
    // STEP 3: Analyse let and def expressions.
    // By this point, all trait types and let types have been established, making it possible to
    // analyse any expression.
    // ==========================================================================================

    // Analyse global let expressions.
    for let_statement in ast.lets.iter() {
        let path = path(&let_statement.name);

        let lett = Let::analyse(let_statement, &context)?;

        context.lets.resolve(&path)?.replace(lett);
    }

    // Analyse module let expressions.
    for module in ast.mods.iter() {
        for let_statement in module.lets.iter() {
            let path = path(&format!("{}\\{}", module.name, let_statement.name));

            let lett = Let::analyse(let_statement, &context)?;

            context.lets.resolve(&path)?.replace(lett);
        }
    }

    // Analyse struct and class constructor and def expressions.
    for module in ast.mods.iter() {
        let path = path(&module.name);

        if let Some(struct_statement) = &module.strukt {
            let strukt = Struct::analyse(struct_statement, &module.defs, &context)?;

            context.lets.resolve(&path)?.replace(strukt.constructor());
        } else if let Some(class_statement) = &module.class {
            let class = Class::analyse(class_statement, &module.defs, &context)?;

            context.lets.resolve(&path)?.replace(class.constructor());
        }
    }

    Ok(context)
}
