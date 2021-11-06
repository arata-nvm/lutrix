use crate::sat::types::*;

use super::solver::Solver;

// ref: https://en.wikipedia.org/wiki/Tseytin_transformation

pub fn not(solver: &mut Solver, dst: Literal, src: Literal) {
    solver.add_clause(&[-dst, -src]);
    solver.add_clause(&[dst, src]);
}

pub fn and(solver: &mut Solver, dst: Literal, src1: Literal, src2: Literal) {
    solver.add_clause(&[-src1, -src2, dst]);
    solver.add_clause(&[src1, -dst]);
    solver.add_clause(&[src2, -dst]);
}

pub fn or(solver: &mut Solver, dst: Literal, src1: Literal, src2: Literal) {
    solver.add_clause(&[src1, src2, -dst]);
    solver.add_clause(&[-src1, dst]);
    solver.add_clause(&[-src2, dst]);
}

pub fn or_many(solver: &mut Solver, dst: Literal, src: &[Literal]) {
    let mut clause1 = src.to_vec();
    clause1.push(-dst);
    solver.add_clause(&clause1);

    for l in src {
        solver.add_clause(&[-l.clone(), dst]);
    }
}

pub fn xor(solver: &mut Solver, dst: Literal, src1: Literal, src2: Literal) {
    solver.add_clause(&[-src1, -src2, -dst]);
    solver.add_clause(&[src1, src2, -dst]);
    solver.add_clause(&[src1, -src2, dst]);
    solver.add_clause(&[-src1, src2, dst]);
}

pub fn half_adder(solver: &mut Solver, sum: Literal, carry: Literal, src1: Literal, src2: Literal) {
    xor(solver, sum, src1, src2);
    and(solver, carry, src1, src2);
}
