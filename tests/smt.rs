use lutrix::smt::Solver;
use lutrix::{int, op};

#[test]
fn bvnot() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 2);
    let b = s.new_variable("x2", 2);
    s.assert(op!(a == int!(0b01, 2)));
    s.assert(op!(a == op!(!b)));
    s.assert(op!(b == int!(0b10, 2)));
    assert!(s.check());
}

#[test]
fn bvand() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == int!(0b0101, 4)));
    s.assert(op!(c == op!(a & b)));
    s.assert(op!(c == int!(0b0001, 4)));
    assert!(s.check());
}

#[test]
fn bvor() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == int!(0b0101, 4)));
    s.assert(op!(c == op!(a | b)));
    s.assert(op!(c == int!(0b0111, 4)));
    assert!(s.check());
}

#[test]
fn bvxor() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == int!(0b0101, 4)));
    s.assert(op!(c == op!(a ^ b)));
    s.assert(op!(c == int!(0b0110, 4)));
    assert!(s.check());
}

#[test]
fn bvadd() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0111, 4)));
    s.assert(op!(b == int!(0b0001, 4)));
    s.assert(op!(c == op!(a + b)));
    s.assert(op!(c == int!(0b1000, 4)));
    assert!(s.check());
}

#[test]
fn bvsub() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0100, 4)));
    s.assert(op!(b == int!(0b0001, 4)));
    s.assert(op!(c == op!(a - b)));
    s.assert(op!(c == int!(0b0011, 4)));
    assert!(s.check());
}
#[test]
fn bvmul() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    let c = s.new_variable("x3", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == int!(0b0100, 4)));
    s.assert(op!(c == op!(a * b)));
    s.assert(op!(c == int!(0b1100, 4)));
    assert!(s.check());
}

#[test]
fn bvshl() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == op!(a << 3)));
    s.assert(op!(b == int!(0b1000, 4)));
    assert!(s.check());
}
#[test]
fn bvshr() {
    let mut s = Solver::new();
    let a = s.new_variable("x1", 4);
    let b = s.new_variable("x2", 4);
    s.assert(op!(a == int!(0b0011, 4)));
    s.assert(op!(b == op!(a >> 1)));
    s.assert(op!(b == int!(0b0001, 4)));
    assert!(s.check());
}

#[test]
fn bvult() {
    let mut s = Solver::new();
    let c0 = int!(0, 1);
    let c1 = int!(1, 1);
    s.assert(op!(c0 < c1));
    assert!(s.check());
}

#[test]
fn bvule() {
    let mut s = Solver::new();
    let c1 = int!(1, 1);
    s.assert(op!(c1 <= c1));
    assert!(s.check());
}

#[test]
fn bvugt() {
    let mut s = Solver::new();
    let c0 = int!(0, 1);
    let c1 = int!(1, 1);
    s.assert(op!(c1 > c0));
    assert!(s.check());
}

#[test]
fn bvuge() {
    let mut s = Solver::new();
    let c1 = int!(1, 1);
    s.assert(op!(c1 >= c1));
    assert!(s.check());
}
