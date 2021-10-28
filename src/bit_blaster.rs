use std::collections::HashMap;

use crate::{ast::*, types::*};

pub fn transform(problem: Problem) -> Cnf {
    let transformer = Transformer::new();
    transformer.transform(problem)
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

    fn transform(mut self, problem: Problem) -> Cnf {
        for stmt in problem {
            self.transform_stmt(stmt);
        }

        self.formula
    }

    fn transform_stmt(&mut self, stmt: Statement) {
        match stmt {
            Statement::Assert(expr) => {
                let expr = self.transform_expr(expr);
                self.assert(expr);
            }
        }
    }

    fn assert(&mut self, expr: Literal) {
        self.add_clause(&[expr]);
    }

    fn transform_expr(&mut self, expr: Expression) -> Literal {
        match expr {
            Expression::Variable(name) => self.variable(name),
            Expression::Not(expr) => {
                let expr = self.transform_expr(*expr);
                self.not(expr)
            }

            Expression::And(expr1, expr2) => {
                let expr1 = self.transform_expr(*expr1);
                let expr2 = self.transform_expr(*expr2);
                self.and(expr1, expr2)
            }
            Expression::Eq(expr1, expr2) => {
                let expr1 = self.transform_expr(*expr1);
                let expr2 = self.transform_expr(*expr2);
                self.eq(expr1, expr2)
            }
            Expression::Or(expr1, expr2) => {
                let expr1 = self.transform_expr(*expr1);
                let expr2 = self.transform_expr(*expr2);
                self.or(expr1, expr2)
            }
            Expression::Xor(expr1, expr2) => {
                let expr1 = self.transform_expr(*expr1);
                let expr2 = self.transform_expr(*expr2);
                self.xor(expr1, expr2)
            }
        }
    }

    fn variable(&mut self, name: String) -> Literal {
        if !self.variables.contains_key(&name) {
            let next_literal = self.next_literal();
            self.variables.insert(name.clone(), next_literal);
        }

        self.variables[&name]
    }

    fn not(&mut self, expr: Literal) -> Literal {
        let dst = self.next_literal();
        self.add_clause(&[dst.inverted(), expr.inverted()]);
        self.add_clause(&[dst, expr]);
        dst
    }

    fn and(&mut self, expr1: Literal, expr2: Literal) -> Literal {
        let dst = self.next_literal();
        self.add_clause(&[expr1.inverted(), expr2.inverted(), dst]);
        self.add_clause(&[expr1, dst.inverted()]);
        self.add_clause(&[expr2, dst.inverted()]);
        dst
    }

    fn eq(&mut self, expr1: Literal, expr2: Literal) -> Literal {
        let tmp = self.xor(expr1, expr2);
        self.not(tmp)
    }

    fn or(&mut self, expr1: Literal, expr2: Literal) -> Literal {
        let dst = self.next_literal();
        self.add_clause(&[expr1, expr2, dst.inverted()]);
        self.add_clause(&[expr1.inverted(), dst]);
        self.add_clause(&[expr2.inverted(), dst]);
        dst
    }

    fn xor(&mut self, expr1: Literal, expr2: Literal) -> Literal {
        let dst = self.next_literal();
        self.add_clause(&[expr1.inverted(), expr2.inverted(), dst.inverted()]);
        self.add_clause(&[expr1, expr2, dst.inverted()]);
        self.add_clause(&[expr1, expr2.inverted(), dst]);
        self.add_clause(&[expr1.inverted(), expr2, dst]);
        dst
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
