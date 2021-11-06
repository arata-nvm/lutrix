use super::types::{Cnf, Literal, Model};

pub struct Solver {
    formula: Cnf,
    models: Vec<Model>,
    literal_index: usize,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            formula: Cnf::new(),
            models: vec![Model::new()],
            literal_index: 0,
        }
    }

    pub fn new_literal(&mut self) -> Literal {
        self.literal_index += 1;
        Literal::new(self.literal_index, false)
    }

    pub fn set_formula(&mut self, formula: Cnf) {
        self.formula = formula;
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

    pub fn model(&self) -> Model {
        let mut model = Model::new();

        for i in 1..=self.literal_index {
            model.insert(i, false);
        }

        for m in &self.models {
            model.extend(m);
        }

        model
    }

    fn apply_unit_rule(&mut self) {
        for literal in self.formula.find_unit_clauses() {
            self.formula.remove_clauses_which_has(&literal);
            self.formula.remove_from_all(&literal.inverted());
            self.cur_model().insert(literal.var, !literal.inverted);
        }
    }

    fn apply_splitting_rule(&mut self) -> bool {
        let literal = match self.formula.head_literal() {
            Some(l) => l,
            None => return false,
        };

        let original = self.formula.clone();

        self.formula.add_clause(&[literal]);
        self.push_model();
        if self.check() {
            return true;
        }
        self.pop_model();
        self.formula = original.clone();

        self.formula.add_clause(&[-literal]);
        self.push_model();
        if self.check() {
            return true;
        }
        self.pop_model();
        self.formula = original;

        false
    }

    fn push_model(&mut self) {
        self.models.push(Model::new());
    }

    fn pop_model(&mut self) {
        self.models.pop();
    }

    fn cur_model(&mut self) -> &mut Model {
        self.models.last_mut().unwrap()
    }
}
