use crate::*;

#[derive(Debug)]
pub enum Value {
    Expr(Box<Op>),
    Var { var_name: char },
}

#[derive(Debug)]
pub enum Op {
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    Xor(Value, Value),
    Then(Value, Value),
}

impl Op {
    pub fn depth(&self) -> usize {
        use Op::*;
        1+match self {
            Not(x) => x.depth(),
            And(a, b) => std::cmp::max(a.depth(), b.depth()),
            Or(a, b) => std::cmp::max(a.depth(), b.depth()),
            Xor(a, b) => std::cmp::max(a.depth(), b.depth()),
            Then(a, b) => std::cmp::max(a.depth(), b.depth()),
        }
    }
    pub fn expr(self) -> Value {
        Value::Expr(Box::new(self))
    }
    pub fn get_plex(&self, vars: &VarPlex, iter: usize) -> bool {
        use Op::*;
        match self {
            Not(x) => !x.get_plex(vars, iter),
            And(a, b) => a.get_plex(vars, iter) && b.get_plex(vars, iter),
            Or(a, b) => a.get_plex(vars, iter) || b.get_plex(vars, iter),
            Xor(a, b) => a.get_plex(vars, iter) ^ b.get_plex(vars, iter),
            Then(a, b) => {
                let a = a.get_plex(vars, iter);
                let b = b.get_plex(vars, iter);
                !(a && !b)
            }
        }
    }
    pub fn get_var(&self, vars: &SetVars) -> bool {
        use Op::*;
        match self {
            Not(x) => !x.get_var(vars),
            And(a, b) => a.get_var(vars) && b.get_var(vars),
            Or(a, b) => a.get_var(vars) || b.get_var(vars),
            Xor(a, b) => a.get_var(vars) ^ b.get_var(vars),
            Then(a, b) => {
                let a = a.get_var(vars);
                let b = b.get_var(vars);
                !(a && !b)
            }
        }
    }
}


impl Value {
    pub fn depth(&self) -> usize {
        use Value::*;
        match self {
            Expr(op) => op.depth(),
            Var { .. } => 1,
        }
    }
    pub fn get_plex(&self, vars: &VarPlex, iter: usize) -> bool {
        use Value::*;
        match self {
            Expr(op) => op.get_plex(vars, iter),
            Var { var_name } => vars.get_plex(var_name, iter),
        }
    }
    pub fn get_var(&self, vars: &SetVars) -> bool {
        use Value::*;
        match self {
            Expr(op) => op.get_var(vars),
            Var { var_name } => vars.get_var(var_name),
        }
    }
}

