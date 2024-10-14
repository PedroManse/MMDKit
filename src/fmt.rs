use crate::*;
use logic::{Op, Value};
use std::fmt;

impl fmt::Display for table::Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let st = self.expr.fmt();
        writeln!(f, "{st}")?;
        write!(f, "# ")?;
        for var in self.get_vars() {
            write!(f, "{var} ")?;
        }
        write!(f, "\n")?;
        for iter in 0..self.get_iter_count() {
            write!(f, "{iter} ")?;
            for var in self.get_vars() {
                write!(f, "{} ", bool_fmt(self.vars.get_plex(var, iter)))?;
            }
            writeln!(f, "| {}", bool_fmt(self.get_plex(iter)))?;
        }
        Ok(())
    }
}

impl fmt::Display for table::Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = self.expr.fmt_tree(self.expr.depth(), &self);
        write!(f, "{}", t)
    }
}

impl Value {
    pub fn fmt(&self) -> String {
        use Value::*;
        match self {
            Expr(x) => x.fmt(),
            Var { var_name } => {
                format!("{}", var_name)
            }
        }
    }
    pub fn fmt_tree(&self, depth: usize, vars: &SetVars) -> String {
        use Value::*;
        match self {
            Expr(x) => x.fmt_tree(depth, vars),
            Var { var_name } => {
                let (down, up) = ansi_tree_down_up(depth);
                let value = bool_fmt(vars.get_var(var_name));
                format!("{down}{value}{up}{var_name}")
            }
        }
    }
}

fn ansi_tree_down_up(depth: usize) -> (String, String) {
    (
        format!("\x1b[{}B", depth),
        format!("\x1b[{}A\x1b[1D", depth),
    )
}

impl Op {
    // every call traverses the tree (downwards) again
    // could make an fmt_tree(&self, depth, vars) -> (String, bool)
    pub fn fmt_tree(&self, depth: usize, vars: &SetVars) -> String {
        use Op::*;
        let (down, up) = ansi_tree_down_up(depth);
        match self {
            Not(x) => {
                format!(
                    "{down}{}{up}¬{}",
                    bool_fmt(self.get_var(vars)),
                    x.fmt_tree(depth-1, vars)
                )
            }
            And(a, b) => {
                format!(
                    "({} {down}{}{up}^ { })",
                    a.fmt_tree(depth-1, vars),
                    bool_fmt(self.get_var(vars)),
                    b.fmt_tree(depth-1, vars)
                )
            }
            Or(a, b) => {
                format!(
                    "({} {down}{}{up}v { })",
                    a.fmt_tree(depth-1, vars),
                    bool_fmt(self.get_var(vars)),
                    b.fmt_tree(depth-1, vars)
                )
            }
            Xor(a, b) => {
                format!(
                    "({} {down}{}{up}≠ {})",
                    a.fmt_tree(depth-1, vars),
                    bool_fmt(self.get_var(vars)),
                    b.fmt_tree(depth-1, vars)
                )
            }
            Then(a, b) => {
                format!(
                    "({} {down}{}{up}-> {})",
                    a.fmt_tree(depth-1, vars),
                    bool_fmt(self.get_var(vars)),
                    b.fmt_tree(depth-1, vars)
                )
            }
        }
    }
    pub fn fmt(&self) -> String {
        use Op::*;
        match self {
            Not(x) => format!("¬({})", x.fmt()),
            And(a, b) => format!("({} ^ { })", a.fmt(), b.fmt()),
            Or(a, b) => format!("({} v { })", a.fmt(), b.fmt()),
            Xor(a, b) => format!("({} ≠ {})", a.fmt(), b.fmt()),
            Then(a, b) => format!("({} -> {})", a.fmt(), b.fmt()),
        }
    }
}

fn bool_fmt(b: bool) -> &'static str {
    match b {
        true => "V",
        false => "F",
    }
}
