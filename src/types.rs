use std::collections::HashMap;

pub type Solution = HashMap<String, bool>;

#[derive(Debug, Clone)]
pub struct Cnf {
    pub clauses: Vec<Clause>,
    pub determined: Solution,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub name: String,
    pub inverted: bool,
}

impl Cnf {
    pub fn new(clauses: Vec<Clause>) -> Self {
        Self {
            clauses,
            determined: HashMap::new(),
        }
    }

    pub fn is_consistent(&self) -> bool {
        self.clauses.is_empty()
    }

    pub fn has_empty_clause(&self) -> bool {
        self.clauses.iter().any(|c| c.is_empty())
    }

    pub fn find_unit_clauses(&self) -> Vec<Literal> {
        self.clauses
            .iter()
            .filter(|c| c.literals.len() == 1)
            .map(|c| c.literals[0].clone())
            .collect()
    }

    pub fn head_literal(&self) -> Option<Literal> {
        self.clauses.get(0).and_then(|c| c.literals.get(0)).cloned()
    }

    pub fn remove_clauses_which_has(&mut self, literal: &Literal) {
        self.clauses.retain(|c| !c.has_literal(literal));
    }

    pub fn remove_from_all(&mut self, literal: &Literal) {
        for i in 0..self.clauses.len() {
            self.clauses[i].remove(literal);
        }
    }

    pub fn assume(&mut self, literal: Literal) {
        self.clauses.push(Clause::new(vec![literal]));
    }

    pub fn determine(&mut self, name: String, value: bool) {
        self.determined.insert(name, value);
    }
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn has_literal(&self, literal: &Literal) -> bool {
        self.literals.iter().any(|l| l == literal)
    }

    pub fn remove(&mut self, literal: &Literal) {
        self.literals.retain(|l| l != literal);
    }
}

impl Literal {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        match name.chars().nth(0).unwrap() {
            '-' => Self {
                name: name[1..].to_string(),
                inverted: true,
            },
            _ => Self {
                name,
                inverted: false,
            },
        }
    }

    pub fn inverted(&self) -> Self {
        Self {
            name: self.name.clone(),
            inverted: !self.inverted,
        }
    }
}
