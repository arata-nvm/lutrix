# lutrix

SAT/SMT solver written in Rust

## Example

solve `x^2 - 6*x + 9 = 0`

```rust
use lutrix::smt;
use lutrix::{int, op};

fn main() {
    let mut s = smt::Solver::new();
    let x = s.new_variable("x", 8);

    let expr = op!(+ op!(- op!(* x, x), op!(* x, int!(6, 8))), int!(9, 8));
    s.assert(op!(= expr, int!(0, 8)));
    s.assert(op!(< x, int!(0xf, 8)));
    assert!(s.check());

    let model = s.model();
    assert_eq!(model["x"], 3);
}
```