#[derive(Debug)]
pub struct Plexer {
    var_count: usize,
    iter_matrix: Vec<Vec<bool>>,
}

impl Plexer {
    pub fn new(var_count: usize) -> Plexer {
        let iter_count = usize::pow(2, var_count as u32);
        let mut iter_matrix = vec![Vec::with_capacity(iter_count); var_count];

        for var in 0..var_count {
            let mut now = true; // current value
            let flip_target = 1 << var_count >> var; // after target iterations, flip current value
            for iter in 1..=iter_count {
                iter_matrix[var].push(now);
                if iter % flip_target == 0 {
                    now = !now;
                }
            }
        }

        Plexer {
            var_count,
            iter_matrix,
        }
    }
    pub fn get_plex(&self, idx: usize, iter: usize) -> bool {
        self.iter_matrix[idx][iter]
    }
    pub fn get_var_count(&self) -> usize {
        self.var_count
    }
    pub fn get_iter_count(&self) -> usize {
        usize::pow(2, self.var_count as u32)
    }
}

// variables "multiplexed" acording to each possible iteration (row) in a truth table
pub struct Vars {
    var_names: Vec<char>,
    plexer: Plexer,
}

impl Vars {
    pub fn new(vars: &[char]) -> Vars {
        Vars {
            var_names: vars.to_vec(),
            plexer: Plexer::new(vars.len()),
        }
    }
    pub fn get_var_name(&self, var_id: usize) -> char {
        self.var_names[var_id]
    }
}

impl std::ops::Deref for Vars {
    type Target = Plexer;
    fn deref(&self) -> &Plexer {
        &self.plexer
    }
}
