use crate::sat::{Literal, Model};

#[derive(Debug, Clone)]
pub struct BitVector {
    pub literals: Vec<Literal>,
}

impl BitVector {
    pub fn new(literals: Vec<Literal>) -> Self {
        Self { literals }
    }

    pub fn new_bool(literal: Literal) -> Self {
        Self {
            literals: vec![literal],
        }
    }

    pub fn as_bool(&self) -> Literal {
        self.literals[0]
    }

    pub fn len(&self) -> usize {
        self.literals.len()
    }

    pub fn at(&self, i: usize) -> Literal {
        self.literals[i]
    }

    pub fn get_int(&self, model: &Model) -> usize {
        self.literals
            .iter()
            .map(|l| if model[&l.var] { 1 } else { 0 })
            .rev()
            .enumerate()
            .map(|(i, val)| val << i)
            .sum()
    }
}
