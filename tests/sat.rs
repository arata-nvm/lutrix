use lutrix::sat::solver::Solver;

#[test]
fn dpll_1() {
    let mut s = Solver::new();
    assert!(s.check());
}

#[test]
fn dpll_2() {
    let mut s = Solver::new();
    let a = s.new_literal();
    s.add_clause(&[a]);
    assert!(s.check());
}

#[test]
fn dpll_3() {
    let mut s = Solver::new();
    let a = s.new_literal();
    s.add_clause(&[a]);
    s.add_clause(&[-a]);
    assert!(!s.check());
}

#[test]
fn dpll_4() {
    let mut s = Solver::new();
    let a = s.new_literal();
    let b = s.new_literal();
    s.add_clause(&[a, b]);
    assert!(s.check());
}

#[test]
fn dpll_5() {
    let mut s = Solver::new();
    let a = s.new_literal();
    let b = s.new_literal();
    s.add_clause(&[a, -b]);
    s.add_clause(&[-a, b]);
    assert!(s.check());
}

#[test]
fn dpll_6() {
    let mut s = Solver::new();
    let a = s.new_literal();
    let b = s.new_literal();
    let c = s.new_literal();
    s.add_clause(&[a, b]);
    s.add_clause(&[-a, c]);
    s.add_clause(&[-a, -c]);
    s.add_clause(&[-b, -c]);
    assert!(s.check());
}
