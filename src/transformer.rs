use crate::types::*;

// ref: https://en.wikipedia.org/wiki/Tseytin_transformation

pub fn not(formula: &mut Cnf, dst: Literal, src: Literal) {
    formula.add_clause(&[dst.inverted(), src.inverted()]);
    formula.add_clause(&[dst, src]);
}

pub fn and(formula: &mut Cnf, dst: Literal, src1: Literal, src2: Literal) {
    formula.add_clause(&[src1.inverted(), src2.inverted(), dst]);
    formula.add_clause(&[src1, dst.inverted()]);
    formula.add_clause(&[src2, dst.inverted()]);
}

pub fn or(formula: &mut Cnf, dst: Literal, src1: Literal, src2: Literal) {
    formula.add_clause(&[src1, src2, dst.inverted()]);
    formula.add_clause(&[src1.inverted(), dst]);
    formula.add_clause(&[src2.inverted(), dst]);
}

pub fn or_many(formula: &mut Cnf, dst: Literal, src: &[Literal]) {
    let mut clause1 = src.to_vec();
    clause1.push(dst.inverted());
    formula.add_clause(&clause1);

    for l in src {
        formula.add_clause(&[l.inverted(), dst]);
    }
}

pub fn xor(formula: &mut Cnf, dst: Literal, src1: Literal, src2: Literal) {
    formula.add_clause(&[src1.inverted(), src2.inverted(), dst.inverted()]);
    formula.add_clause(&[src1, src2, dst.inverted()]);
    formula.add_clause(&[src1, src2.inverted(), dst]);
    formula.add_clause(&[src1.inverted(), src2, dst]);
}

pub fn half_adder(formula: &mut Cnf, sum: Literal, carry: Literal, src1: Literal, src2: Literal) {
    xor(formula, sum, src1, src2);
    and(formula, carry, src1, src2);
}
