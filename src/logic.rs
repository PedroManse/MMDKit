use crate::*;

pub enum Value {
    Expr(Box<Op>),
    Var { var_index: usize },
}

pub enum Op {
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Then(Value, Value),
}

impl Op {
    pub fn expr(self) -> Value {
        Value::Expr(Box::new(self))
    }
}
