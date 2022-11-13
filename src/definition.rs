use crate::expression::Expression;

#[derive(Clone)]
pub struct Definition {
    // Inputs and outputs are declared by the trait
    pub expression: Expression,
}
