use crate::*;
use logic::{Op, Value};

use std::fmt;
impl fmt::Display for table::Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let st = self.expr.fmt_plex(&self.vars);
        f.write_str(&st)
    }
}

impl Value {
    pub fn fmt_plex(&self, vars: &Vars) -> String {
        use Value::*;
        match self {
            Expr(x) => x.fmt_plex(vars),
            Var { var_index } => {
                format!("({})", vars.get_var_name(*var_index).to_string())
            }
        }
    }
}

impl Op {
    pub fn fmt_plex(&self, vars: &Vars) -> String {
        use Op::*;
        match self {
            Not(x) => format!("¬({})", x.fmt_plex(vars)),
            And(a, b) => format!("({} ^ { })", a.fmt_plex(vars), b.fmt_plex(vars)),
            Or(a, b) => format!("({} v { })", a.fmt_plex(vars), b.fmt_plex(vars)),
            Xor(a, b) => format!("({} ≠ {})", a.fmt_plex(vars), b.fmt_plex(vars)),
            Then(a, b) => format!("({} -> {})", a.fmt_plex(vars), b.fmt_plex(vars)),
        }
    }
}
