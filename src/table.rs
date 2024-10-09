use crate::*;
use logic::{Op, Value};

impl Value {
    pub fn get_plex(&self, vars: &Vars, iter: usize) -> bool {
        use Value::*;
        match self {
            Expr(op) => op.get_plex(vars, iter),
            Var { var_index } => vars.get_plex(*var_index, iter),
        }
    }
}

impl Op {
    pub fn get_plex(&self, vars: &Vars, iter: usize) -> bool {
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
}

pub struct Table {
    pub expr: Value,
    pub vars: Vars,
}

impl Table {
    pub fn new(expr: Value, vars: Vars) -> Table {
        Table { expr, vars }
    }
}
