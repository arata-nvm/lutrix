use std::collections::HashMap;

use crate::sat;

use super::{ast::Expression, bit_vector::BitVector};

pub type Model = HashMap<String, usize>;

pub struct Solver {
    pub sat_solver: sat::Solver,
    pub variables: HashMap<String, BitVector>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            sat_solver: sat::Solver::new(),
            variables: HashMap::new(),
        }
    }

    pub fn new_variable<S: Into<String>>(&mut self, name: S, len: usize) -> Expression {
        let name = name.into();

        let literals = (0..len).map(|_| self.sat_solver.new_literal()).collect();
        let bv = BitVector::new(literals);
        self.variables.insert(name.clone(), bv);
        Expression::Variable(name)
    }

    pub fn assert(&mut self, expr: Expression) {
        let val = self.transform(expr);
        self.sat_solver.add_clause(&[val.as_bool()]);
    }

    pub fn add_clause(&mut self, literals: &[sat::Literal]) {
        self.sat_solver.add_clause(literals);
    }

    pub fn check(&mut self) -> bool {
        self.sat_solver.check()
    }

    pub fn model(&self) -> Model {
        let model = self.sat_solver.model();
        self.variables
            .iter()
            .map(|(name, bv)| (name.clone(), bv.get_int(&model)))
            .collect()
    }
}
