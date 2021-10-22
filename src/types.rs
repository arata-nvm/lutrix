use std::{collections::HashMap, fmt};

pub type Variable = usize;
pub type Solution = HashMap<Variable, bool>;

#[derive(Debug, Clone)]
pub struct Cnf {
    pub num_of_variables: usize,
    pub clauses: Vec<Clause>,
    pub determined: Solution,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Literal {
    pub var: Variable,
    pub inverted: bool,
}

impl Cnf {
    pub fn new(num_of_variables: usize) -> Self {
        Self {
            num_of_variables,
            clauses: Vec::new(),
            determined: {
                let mut determined = Solution::new();
                for i in 1..=num_of_variables {
                    determined.insert(i, false);
                }
                determined
            },
        }
    }

    pub fn add_clause(&mut self, literals: &[Literal]) {
        self.clauses.push(Clause::new(literals.to_vec()));
    }

    pub fn remove_clauses_which_has(&mut self, literal: &Literal) {
        self.clauses.retain(|c| !c.has_literal(literal));
    }

    pub fn remove_from_all(&mut self, literal: &Literal) {
        for i in 0..self.clauses.len() {
            self.clauses[i].remove(literal);
        }
    }

    pub fn determine(&mut self, var: Variable, value: bool) {
        self.determined.insert(var, value);
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
}

impl Clause {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn remove(&mut self, literal: &Literal) {
        self.literals.retain(|l| l != literal);
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }

    pub fn has_literal(&self, literal: &Literal) -> bool {
        self.literals.iter().any(|l| l == literal)
    }
}

impl Literal {
    pub fn new(var: Variable, inverted: bool) -> Self {
        assert!(var != 0);
        Self { var, inverted }
    }

    pub fn inverted(&self) -> Self {
        Self {
            var: self.var,
            inverted: !self.inverted,
        }
    }
}

impl fmt::Display for Cnf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.clauses
                .iter()
                .map(|c| format!("{}", c))
                .collect::<Vec<String>>()
                .join(" && ")
        )
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.literals
                .iter()
                .map(|l| format!("{}", l))
                .collect::<Vec<String>>()
                .join(" || ")
        )
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inverted {
            false => write!(f, "x{}", self.var),
            true => write!(f, "!x{}", self.var),
        }
    }
}
