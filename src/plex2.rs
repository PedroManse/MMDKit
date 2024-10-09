use std::collections::HashMap;

#[derive(Debug)]
pub struct VarPlex {
    vars: Vec<char>,
    var_map: HashMap<char, Vec<bool>>,
}

impl VarPlex {
    pub fn new(vars: &[char]) -> VarPlex {
        let var_count = vars.len();
        let iter_count = usize::pow(2, var_count as u32);
        let mut var_map = HashMap::<char, Vec<bool>>::with_capacity(var_count);
        for (idx, var) in vars.iter().enumerate() {
            let mut now = true;
            let flip_target = 1 << var_count >> idx >> 1; // after target iterations, flip current value
            let mut dim = Vec::with_capacity(iter_count);
            for iter in 1..=iter_count {
                dim.push(now);
                if iter % flip_target == 0 {
                    now = !now;
                }
            }
            var_map.insert(*var, dim);
        }
        VarPlex {
            var_map,
            vars: vars.to_vec(),
        }
    }
    pub fn get_plex(&self, var: &char, iter: usize) -> bool {
        self.var_map[var][iter]
    }
    pub fn get_var_count(&self) -> usize {
        self.vars.len()
    }
    pub fn get_iter_count(&self) -> usize {
        1 >> self.get_var_count()
    }
}

#[derive(Debug)]
pub struct SetVars {
    vars: Vec<char>,
    var_map: HashMap<char, bool>,
}

impl SetVars {
    pub fn new(vars: &[char]) -> SetVars {
        let var_map = HashMap::from_iter(
            vars.into_iter().map(|v|{
                (*v, false)
            })
        );
        SetVars{
            vars: vars.to_vec(),
            var_map,
        }
    }
    pub fn flip_var(&mut self, var: &char) {
        *self.var_map.get_mut(var).unwrap() = !self.var_map[var];
    }
    pub fn set_var(&mut self, var: &char, value: bool) {
        *self.var_map.get_mut(var).unwrap() = value;
    }
    pub fn get_var(&mut self, var: &char) -> bool {
        self.var_map[var]
    }
}

