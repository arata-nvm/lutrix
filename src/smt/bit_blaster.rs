use crate::{sat::types::*, smt::ast::*, smt::tseytin};

use super::{bit_vector::BitVector, solver::Solver};

impl Solver {
    pub fn transform(&mut self, expr: Expression) -> BitVector {
        match expr {
            Expression::Constant(var, length) => self.constant(var, length),
            Expression::Variable(name) => self.variable(name),
            Expression::Not(expr) => {
                let expr = self.transform(*expr);
                self.not(expr)
            }

            Expression::And(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.and(val1, val2)
            }
            Expression::Eq(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.eq(val1, val2)
            }
            Expression::Or(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.or(val1, val2)
            }
            Expression::Xor(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.xor(val1, val2)
            }

            Expression::BvNot(val1) => {
                let val1 = self.transform(*val1);
                self.bvnot(val1)
            }
            Expression::BvAnd(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvand(val1, val2)
            }
            Expression::BvOr(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvor(val1, val2)
            }
            Expression::BvXor(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvxor(val1, val2)
            }
            Expression::BvAdd(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvadd(val1, val2)
            }
            Expression::BvSub(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvsub(val1, val2)
            }
            Expression::BvMul(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvmul(val1, val2)
            }
            Expression::BvShl(val, n) => {
                let val = self.transform(*val);
                self.bvshl(val, n)
            }
            Expression::BvShr(val, n) => {
                let val = self.transform(*val);
                self.bvshr(val, n)
            }
            Expression::BvUlt(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvult(val1, val2)
            }
            Expression::BvUle(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvule(val1, val2)
            }
            Expression::BvUgt(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvugt(val1, val2)
            }
            Expression::BvUge(val1, val2) => {
                let val1 = self.transform(*val1);
                let val2 = self.transform(*val2);
                self.bvuge(val1, val2)
            }
        }
    }

    fn constant(&mut self, var: usize, length: usize) -> BitVector {
        let tmp = self.next_literals(length);
        for i in 0..length {
            let l = tmp.at(length - i - 1);
            match (var >> i) & 1 {
                0 => self.add_clause(&[-l]),
                1 => self.add_clause(&[l]),
                _ => unreachable!(),
            }
        }
        tmp
    }

    fn variable(&mut self, name: String) -> BitVector {
        match self.variables.get(&name) {
            Some(var) => var.clone(),
            None => panic!("variable `{}` not found", name),
        }
    }

    fn not(&mut self, val: BitVector) -> BitVector {
        let dst = self.next_literal();
        tseytin::not(self, dst.as_bool(), val.as_bool());
        dst
    }

    fn and(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        let dst = self.next_literal();
        tseytin::and(self, dst.as_bool(), val1.as_bool(), val2.as_bool());
        dst
    }

    fn eq(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());
        match val1.len() {
            1 => {
                let tmp = self.xor(val1, val2);
                self.not(tmp)
            }
            _ => {
                let tmp1 = self.bvxor(val1, val2);
                let tmp2 = self.next_literal();
                tseytin::or_many(self, tmp2.as_bool(), &tmp1.literals);
                self.not(tmp2)
            }
        }
    }

    fn or(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        let dst = self.next_literal();
        tseytin::or(self, dst.as_bool(), val1.as_bool(), val2.as_bool());
        dst
    }

    fn xor(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        let dst = self.next_literal();
        tseytin::xor(self, dst.as_bool(), val1.as_bool(), val2.as_bool());
        dst
    }

    fn bvnot(&mut self, val: BitVector) -> BitVector {
        let dst = self.next_literals(val.len());
        for i in 0..dst.len() {
            tseytin::not(self, dst.at(i), val.at(i));
        }

        dst
    }

    fn bvand(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for i in 0..dst.len() {
            tseytin::and(self, dst.at(i), val1.at(i), val2.at(i));
        }

        dst
    }

    fn bvor(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for i in 0..dst.len() {
            tseytin::or(self, dst.at(i), val1.at(i), val2.at(i));
        }

        dst
    }

    fn bvxor(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        for i in 0..dst.len() {
            tseytin::xor(self, dst.at(i), val1.at(i), val2.at(i));
        }

        dst
    }

    fn bvadd(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let dst = self.next_literals(val1.len());
        let mut carry = self.next_literal().as_bool();
        self.add_clause(&[carry.inverted()]);

        for i in (0..dst.len()).rev() {
            let new_carry = self.next_literal().as_bool();
            self.full_adder(dst.at(i), new_carry, val1.at(i), val2.at(i), carry);
            carry = new_carry;
        }
        dst
    }

    pub fn bvsub(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let val2_not = self.bvnot(val2);
        let one = self.constant(1, val1.len());
        let val2_comp = self.bvadd(val2_not, one);
        self.bvadd(val1, val2_comp)
    }

    pub fn bvmul(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let mut dst = self.constant(0, val1.len());
        for i in 0..val1.len() {
            let tmp = self.bvshl(val1.clone(), i);
            let tmp2 = self.next_literals(val1.len());
            for j in 0..val1.len() {
                tseytin::and(self, tmp2.at(j), tmp.at(j), val2.at(val1.len() - i - 1));
            }

            dst = self.bvadd(dst, tmp2);
        }
        dst
    }

    pub fn bvshl(&mut self, val: BitVector, n: usize) -> BitVector {
        let dst = self.next_literals(val.len());

        if val.len() >= n {
            for i in 0..(val.len() - n) {
                let eq = self.eq(
                    BitVector::new_bool(dst.at(i)),
                    BitVector::new_bool(val.at(i + n)),
                );
                self.add_clause(&[eq.as_bool()]);
            }
        }
        for i in 1..=n.min(val.len()) {
            self.add_clause(&[dst.at(val.len() - i).inverted()]);
        }

        dst
    }

    pub fn bvshr(&mut self, val: BitVector, n: usize) -> BitVector {
        let dst = self.next_literals(val.len());

        for i in n..val.len() {
            let eq = self.eq(
                BitVector::new_bool(dst.at(i)),
                BitVector::new_bool(val.at(i - n)),
            );
            self.add_clause(&[eq.as_bool()]);
        }
        for i in 0..n.min(val.len()) {
            self.add_clause(&[dst.at(i).inverted()]);
        }

        dst
    }

    pub fn bvult(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        // TODO
        let tmp = self.bvsub(val1, val2);
        BitVector::new_bool(tmp.at(0))
    }

    pub fn bvule(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        let tmp1 = self.bvult(val1.clone(), val2.clone());
        let tmp2 = self.eq(val1, val2);
        self.or(tmp1, tmp2)
    }

    pub fn bvugt(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        self.bvult(val2, val1)
    }

    pub fn bvuge(&mut self, val1: BitVector, val2: BitVector) -> BitVector {
        assert_eq!(val1.len(), val2.len());

        // TODO
        self.bvule(val2, val1)
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

        tseytin::half_adder(self, s1, c1, src1, src2);
        tseytin::half_adder(self, sum, c2, s1, prev_carry);
        tseytin::or(self, carry, c1, c2);
    }

    fn next_literal(&mut self) -> BitVector {
        BitVector::new_bool(self.sat_solver.new_literal())
    }

    fn next_literals(&mut self, length: usize) -> BitVector {
        let literals = (0..length).map(|_| self.next_literal().as_bool()).collect();
        BitVector::new(literals)
    }
}
