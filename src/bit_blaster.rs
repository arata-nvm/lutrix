use std::collections::HashMap;

use crate::{ast::*, transformer, types::*};

type TransformedProblem = (Cnf, HashMap<String, Value>);

pub fn transform(problem: Problem) -> TransformedProblem {
    let mut transformer = Transformer::new();
    transformer.transform(problem);
    (transformer.formula, transformer.variables)
}

struct Transformer {
    formula: Cnf,
    variables: HashMap<String, Value>,
    literal_index: usize,
}

#[derive(Debug, Clone)]
pub enum Value {
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

    fn transform(&mut self, problem: Problem) {
        for stmt in problem {
            self.transform_stmt(stmt);
        }
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
            VariableType::BitVector(length) => self.next_literals(length),
        };
        self.variables.insert(name, val);
    }

    fn transform_expr(&mut self, expr: Expression) -> Value {
        match expr {
            Expression::Constant(var, length) => self.constant(var, length),
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

            Expression::BvNot(val1) => {
                let val1 = self.transform_expr(*val1);
                self.bvnot(val1)
            }
            Expression::BvAnd(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.bvand(val1, val2)
            }
            Expression::BvOr(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.bvor(val1, val2)
            }
            Expression::BvXor(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.bvxor(val1, val2)
            }
            Expression::BvAdd(val1, val2) => {
                let val1 = self.transform_expr(*val1);
                let val2 = self.transform_expr(*val2);
                self.bvadd(val1, val2)
            }
        }
    }

    fn constant(&mut self, var: usize, length: usize) -> Value {
        let tmp = self.next_literals(length);
        for (i, l) in tmp.as_bv().into_iter().rev().enumerate() {
            match (var >> i) & 1 {
                0 => self.add_clause(&[l.inverted()]),
                1 => self.add_clause(&[l]),
                _ => unreachable!(),
            }
        }
        tmp
    }

    fn variable(&mut self, name: String) -> Value {
        match self.variables.get(&name) {
            Some(var) => var.clone(),
            None => panic!("variable `{}` not found", name),
        }
    }

    fn not(&mut self, val: Value) -> Value {
        let dst = self.next_literal();
        transformer::not(&mut self.formula, dst.as_bool(), val.as_bool());
        dst
    }

    fn and(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        transformer::and(
            &mut self.formula,
            dst.as_bool(),
            val1.as_bool(),
            val2.as_bool(),
        );
        dst
    }

    fn eq(&mut self, val1: Value, val2: Value) -> Value {
        match (&val1, &val2) {
            (Value::Bool(_), Value::Bool(_)) => {
                let tmp = self.xor(val1, val2);
                self.not(tmp)
            }
            (Value::BitVector(bv1), Value::BitVector(bv2)) => {
                assert_eq!(bv1.len(), bv2.len());
                let tmp1 = self.bvxor(val1, val2);
                let tmp2 = self.next_literal();
                transformer::or_many(&mut self.formula, tmp2.as_bool(), &tmp1.as_bv());
                self.not(tmp2)
            }

            _ => panic!(),
        }
    }

    fn or(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        transformer::or(
            &mut self.formula,
            dst.as_bool(),
            val1.as_bool(),
            val2.as_bool(),
        );
        dst
    }

    fn xor(&mut self, val1: Value, val2: Value) -> Value {
        let dst = self.next_literal();
        transformer::xor(
            &mut self.formula,
            dst.as_bool(),
            val1.as_bool(),
            val2.as_bool(),
        );
        dst
    }

    fn bvnot(&mut self, val: Value) -> Value {
        let val = val.as_bv();

        let dst = self.next_literals(val.len());
        for (i, d) in dst.as_bv().into_iter().enumerate() {
            transformer::not(&mut self.formula, d, val[i]);
        }

        dst
    }

    fn bvand(&mut self, val1: Value, val2: Value) -> Value {
        let val1 = val1.as_bv();
        let val2 = val2.as_bv();
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for (i, d) in dst.as_bv().into_iter().enumerate() {
            transformer::and(&mut self.formula, d, val1[i], val2[i]);
        }

        dst
    }

    fn bvor(&mut self, val1: Value, val2: Value) -> Value {
        let val1 = val1.as_bv();
        let val2 = val2.as_bv();
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for (i, d) in dst.as_bv().into_iter().enumerate() {
            transformer::or(&mut self.formula, d, val1[i], val2[i]);
        }

        dst
    }

    fn bvxor(&mut self, val1: Value, val2: Value) -> Value {
        let val1 = val1.as_bv();
        let val2 = val2.as_bv();
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for (i, d) in dst.as_bv().into_iter().enumerate() {
            transformer::xor(&mut self.formula, d, val1[i], val2[i]);
        }

        dst
    }

    fn bvadd(&mut self, val1: Value, val2: Value) -> Value {
        let val1 = val1.as_bv();
        let val2 = val2.as_bv();
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        let mut carry = self.next_literal().as_bool();
        self.add_clause(&[carry.inverted()]);

        for (i, d) in dst.as_bv().into_iter().enumerate().rev() {
            let new_carry = self.next_literal().as_bool();
            self.full_adder(d, new_carry, val1[i], val2[i], carry);
            carry = new_carry;
        }
        dst
    }

    pub fn full_adder(
        &mut self,
        sum: Literal,
        carry: Literal,
        src1: Literal,
        src2: Literal,
        prev_carry: Literal,
    ) {
        let s1 = self.next_literal().as_bool();
        let c1 = self.next_literal().as_bool();
        let c2 = self.next_literal().as_bool();

        transformer::half_adder(&mut self.formula, s1, c1, src1, src2);
        transformer::half_adder(&mut self.formula, sum, c2, s1, prev_carry);
        transformer::or(&mut self.formula, carry, c1, c2);
    }

    fn add_clause(&mut self, literals: &[Literal]) {
        self.formula.add_clause(literals);
    }

    fn next_literal(&mut self) -> Value {
        let new_index = self.literal_index;
        self.literal_index += 1;
        Value::Bool(Literal::new(new_index, false))
    }

    fn next_literals(&mut self, length: usize) -> Value {
        let bv = (1..length).map(|_| self.next_literal().as_bool()).collect();
        Value::BitVector(bv)
    }
}
