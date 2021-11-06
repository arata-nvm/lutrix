use std::{collections::HashMap, fmt, ops::Neg};

pub type Variable = usize;
pub type Model = HashMap<Variable, bool>;

#[derive(Debug, Clone)]
pub struct Cnf {
    pub clauses: Vec<Clause>,
}

#[derive(Debug, Clone)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Literal {
    pub var: Variable,
    pub inverted: bool,
}

impl Cnf {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
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
            .map(|c| c.literals[0])
            .collect()
    }

    pub fn head_literal(&self) -> Option<Literal> {
        self.clauses.get(0).and_then(|c| c.literals.get(0)).cloned()
    }

    pub fn dump(&self) -> String {
        let mut num_of_var = 0;
        let mut buf = String::new();
        for clause in &self.clauses {
            for literal in &clause.literals {
                if literal.inverted {
                    buf.push_str("-");
                }
                buf.push_str(&format!("{} ", literal.var));
                num_of_var = num_of_var.max(literal.var);
            }
            buf.push_str("0\n");
        }

        buf.insert_str(0, &format!("p cnf {} {}\n", num_of_var, self.clauses.len()));
        buf
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

impl Neg for Literal {
    type Output = Literal;
    fn neg(self) -> Self::Output {
        self.inverted()
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
