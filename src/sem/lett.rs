use crate::ast::expression::Expression;
use crate::sem::typ::Type;
use crate::runtime::instance::Instance;
use crate::sem::scope::Scope;
use std::collections::HashMap;
use std::rc::Rc;
use crate::ast::let_statement::LetStatement;

// A 'let' defines a constant instance or a function.
pub struct Let {
    pub inputs: HashMap<String, Type>,
    pub output: Type,
    pub expression: Expression,
}

// TODO: this should go somewhere in the runtime module.
impl Let {
    pub fn resolve(&self, inputs: HashMap<String, Rc<Instance>>, scope: &Scope) -> Rc<Instance> {
        let local_scope = scope.local_scope(None, inputs);

        self.expression.resolve(&local_scope)
    }
}

impl From<LetStatement> for Let {
    fn from(statement: LetStatement) -> Self {
        Let {
            inputs: statement.parameters.into_iter().collect(),
            output: statement.output,
            expression: statement.expr,
        }
    }
}