use crate::*;

#[derive(Debug)]
pub struct Table {
    pub expr: Value,
    pub vars: VarPlex,
    pub results: Vec<bool>,
}

impl Table {
    pub fn new(expr: Value, vars: VarPlex) -> Table {
        let mut results = Vec::with_capacity(vars.get_iter_count());
        for iter in 0..vars.get_iter_count() {
            results.push(expr.get_plex(&vars, iter));
        }
        Table {
            expr,
            vars,
            results,
        }
    }
    pub fn get_plex(&self, iter: usize) -> bool {
        self.results[iter]
    }
}

impl std::ops::Deref for Table {
    type Target = VarPlex;
    fn deref(&self) -> &Self::Target {
        &self.vars
    }
}

#[derive(Debug)]
pub struct Tree {
    pub expr: Value,
    pub vars: SetVars,
}

impl Tree {
    pub fn new(expr: Value, vars: SetVars) -> Tree {
        Tree { expr, vars }
    }
    pub fn resolve(&self) -> bool {
        self.expr.get_var(&self.vars)
    }
}

impl std::ops::Deref for Tree {
    type Target = SetVars;
    fn deref(&self) -> &Self::Target {
        &self.vars
    }
}

