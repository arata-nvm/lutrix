use std::collections::HashMap;

use crate::{ast::*, types::*};

pub fn transform(problem: Problem) -> Cnf {
    let transformer = Transformer::new();
    transformer.transform(problem)
}

struct Transformer {
    formula: Cnf,
    variables: HashMap<String, Value>,
    literal_index: usize,
}

#[derive(Debug, Clone)]
enum Value {
    Bool(Literal),
    BitVector(Vec<Literal>),
}

impl Value {
    fn as_bool(&self) -> Literal {
        match self {
            Value::Bool(literal) => literal.clone(),
            _ => panic!(),
        }
    }

    fn as_bv(&self) -> Vec<Literal> {
        match self {
            Value::BitVector(bv) => bv.clone(),
            _ => panic!(),
        }
    }
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
            Statement::Define(name, typ) => {
                self.define(name, typ);
            }
        }
    }

    fn assert(&mut self, val: Value) {
        self.add_clause(&[val.as_bool()]);
    }

    fn define(&mut self, name: String, typ: VariableType) {
        let val = match typ {
            VariableType::Bool => self.next_literal(),
            VariableType::BitVector(length) => {
                let bv = (1..length).map(|_| self.next_literal().as_bool()).collect();
                Value::BitVector(bv)
            }
        };
        self.variables.insert(name, val);
    }

    fn transform_expr(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Variable(name) => self.variable(name),
            Expression::Not(expr) => {
                let expr = self.transform_expr(*expr);
                self.not(expr)
            }

            Expression::And(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.and(val1, val2)
            }
            Expression::Eq(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.eq(val1, val2)
            }
            Expression::Or(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.or(val1, val2)
            }
            Expression::Xor(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.xor(val1, val2)
            }
        }
    }

    fn variable(&mut self, name: String) -> Value {
        match self.variables.get(&name) {
            Some(var) => var.clone(),
            None => panic!("variable `{}` not found", name),
        }
    }

    fn not(&mut self, val: Value) -> Value {
        let dst = self.next_literal();
        {
            let dst = dst.as_bool();
            let val = val.as_bool();
            self.add_clause(&[dst.inverted(), val.inverted()]);
            self.add_clause(&[dst, val]);
        }
        dst
    }

    fn and(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        {
            let dst = dst.as_bool();
            let val1 = val1.as_bool();
            let val2 = val2.as_bool();
            self.add_clause(&[val1.inverted(), val2.inverted(), dst]);
            self.add_clause(&[val1, dst.inverted()]);
            self.add_clause(&[val2, dst.inverted()]);
        }
        dst
    }

    fn eq(&mut self, val1: Value, val2: Value) -> Value {
        let tmp = self.xor(val1, val2);
        self.not(tmp)
    }

    fn or(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        {
            let dst = dst.as_bool();
            let val1 = val1.as_bool();
            let val2 = val2.as_bool();
            self.add_clause(&[val1, val2, dst.inverted()]);
            self.add_clause(&[val1.inverted(), dst]);
            self.add_clause(&[val2.inverted(), dst]);
        }
        dst
    }

    fn xor(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        {
            let dst = dst.as_bool();
            let val1 = val1.as_bool();
            let val2 = val2.as_bool();
            self.add_clause(&[val1.inverted(), val2.inverted(), dst.inverted()]);
            self.add_clause(&[val1, val2, dst.inverted()]);
            self.add_clause(&[val1, val2.inverted(), dst]);
            self.add_clause(&[val1.inverted(), val2, dst]);
        }
        dst
    }

    fn add_clause(&mut self, literals: &[Literal]) {
        self.formula.add_clause(literals);
    }

    fn next_literal(&mut self) -> Value {
        let new_index = self.literal_index;
        self.literal_index += 1;
        Value::Bool(Literal::new(new_index, false))
    }
}
