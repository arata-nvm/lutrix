use lutrix::smt::Solver;
use lutrix::{int, op};

#[test]
fn solver_1() {
    let mut s = Solver::new();
    let a = s.new_variable("a", 8);
    let b = s.new_variable("b", 8);
    let c = s.new_variable("c", 8);
    s.assert(op!(a == int!(30, 8)));
    s.assert(op!(b == int!(8, 8)));
    s.assert(op!(c == op!(a + b)));
    assert!(s.check());

    let model = s.model();
    assert_eq!(model["a"], 30);
    assert_eq!(model["b"], 8);
    assert_eq!(model["c"], 38);
}
