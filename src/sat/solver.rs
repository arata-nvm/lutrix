use super::types::{Cnf, Literal, Model, Variable};

pub struct Solver {
    formula: Cnf,
    model: Model,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            formula: Cnf::new(0),
            model: Model::new(),
        }
    }

    pub fn add_clause(&mut self, literals: &[Literal]) {
        self.formula.add_clause(literals);
    }

    pub fn check(&mut self) -> bool {
        self.apply_unit_rule();

        if self.formula.is_consistent() {
            return true;
        }
        if self.formula.has_empty_clause() {
            return false;
        }

        self.apply_splitting_rule()
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    fn apply_unit_rule(&mut self) {
        for literal in self.formula.find_unit_clauses() {
            self.formula.remove_clauses_which_has(&literal);
            self.formula.remove_from_all(&literal.inverted());
            self.formula.determine(literal.var, !literal.inverted);
            self.model.insert(literal.var, !literal.inverted);
        }
    }

    fn apply_splitting_rule(&mut self) -> bool {
        let literal = match self.formula.head_literal() {
            Some(l) => l,
            None => return false,
        };

        let original = self.formula.clone();
        let prev_model = self.model.clone();

        self.formula.add_clause(&[literal]);
        if self.check() {
            return true;
        }
        self.formula = original.clone();
        self.model = prev_model.clone();

        self.formula.add_clause(&[literal.inverted()]);
        if self.check() {
            return true;
        }
        self.formula = original;
        self.model = prev_model;

        false
    }
}
