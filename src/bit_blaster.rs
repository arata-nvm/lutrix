use std::collections::HashMap;

use crate::{ast::*, types::*};

pub fn transform(ast: Node) -> Cnf {
    let transformer = Transformer::new();
    transformer.transform(ast)
}

struct Transformer {
    formula: Cnf,
    variables: HashMap<String, Literal>,
    literal_index: usize,
}

impl Transformer {
    fn new() -> Self {
        Self {
            formula: Cnf::new(0),
            variables: HashMap::new(),
            literal_index: 1,
        }
    }

    fn transform(mut self, node: Node) -> Cnf {
        match node {
            Node::Assert(expr) => {
                let expr = self.transform_expr(*expr);
                self.assert(expr);
            }
            _ => panic!(),
        }

        self.formula
    }

    fn assert(&mut self, expr: Literal) {
        self.add_clause(&[expr]);
    }

    fn transform_expr(&mut self, node: Node) -> Literal {
        match node {
            Node::Variable(name) => self.variable(name),
            _ => panic!(),
        }
    }

    fn variable(&mut self, name: String) -> Literal {
        if !self.variables.contains_key(&name) {
            let next_literal = self.next_literal();
            self.variables.insert(name.clone(), next_literal);
        }

        self.variables[&name]
    }

    fn add_clause(&mut self, literals: &[Literal]) {
        self.formula.add_clause(literals);
    }

    fn next_literal(&mut self) -> Literal {
        let new_index = self.literal_index;
        self.literal_index += 1;
        Literal::new(new_index, false)
    }
}
