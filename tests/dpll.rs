use lutrix::{dimacs, sat::dpll};

#[test]
fn dpll_1() {
    let input = "p cnf 0 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_sat());
}

#[test]
fn dpll_2() {
    let input = "p cnf 1 1
                        1 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_sat());
}

#[test]
fn dpll_3() {
    let input = "p cnf 1 2
                        1 0
                        -1 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_unsat());
}

#[test]
fn dpll_4() {
    let input = "p cnf 2 1
                        1 2 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_sat());
}

#[test]
fn dpll_5() {
    let input = "p cnf 2 2
                        1 -2 0
                        -1 2 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_sat());
}

#[test]
fn dpll_6() {
    let input = "p cnf 3 4
                        1 2 0
                        -1 3 0
                        -1 -3 0
                        -2 -3 0";
    let cnf = dimacs::parse(input);
    assert!(dpll::solve(cnf).is_sat());
}
