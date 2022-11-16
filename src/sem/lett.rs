use crate::ast::expression::Expression;
use crate::ast::typ::Type;
use crate::runtime::instance::Instance;
use crate::sem::scope::Scope;
use std::collections::HashMap;
use std::rc::Rc;

// A 'let' defines a constant instance or a function.
pub struct Let {
    pub inputs: HashMap<String, Type>,
    pub outputs: HashMap<String, Type>,
    pub expression: Expression,
}

impl Let {
    pub fn resolve(&self, inputs: HashMap<String, Rc<Instance>>, scope: &Scope) -> Rc<Instance> {
        let local_scope = scope.local_scope(None, inputs);

        self.expression.resolve(&local_scope)
    }
}