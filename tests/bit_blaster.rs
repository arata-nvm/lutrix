use lutrix::{sat::dpll, smt::bit_blaster, smtlib};

#[test]
fn bb_1() {
    let input = "(declare-fun x () Bool)
                (declare-fun y () Bool)
                (assert (not (= (not (or x y)) (and (not x) (not y)))))";
    let problem = smtlib::parse(input);
    let (formula, _) = bit_blaster::transform(problem);
    let result = dpll::solve(formula);
    assert!(result.is_unsat());
}

#[test]
fn bb_2() {
    let input = "(declare-fun x () (_ BitVec 8))
                (declare-fun y () (_ BitVec 8))
                (assert (= x #x05))
                (assert (= y #x03))
                (assert (= (bvadd x y) #x08))";
    let problem = smtlib::parse(input);
    let (formula, _) = bit_blaster::transform(problem);
    let result = dpll::solve(formula);
    assert!(result.is_sat());
}
