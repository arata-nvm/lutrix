use crate::types::*;

pub fn solve(mut cnf: Cnf) -> bool {
    // unit rule
    for literal in cnf.find_unit_clauses() {
        cnf.remove_clauses_which_has(&literal);
        cnf.remove_from_all(&literal.inverted());
    }

    if cnf.is_consistent() {
        return true;
    }
    if cnf.has_empty_clause() {
        return false;
    }

    // splitting rule
    let literal = match cnf.head_literal() {
        Some(l) => l,
        None => return false,
    };

    let mut cnf_true = cnf.clone();
    cnf_true.assume(literal.clone());
    if solve(cnf_true) {
        return true;
    }

    let mut cnf_false = cnf.clone();
    cnf_false.assume(literal.inverted());
    if solve(cnf_false) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::types::*;

    #[test]
    fn dpll_1() {
        let cnf = Cnf::new(vec![]);
        assert_eq!(super::solve(cnf), true);
    }

    #[test]
    fn dpll_2() {
        let cnf = Cnf::new(vec![Clause::new(vec![Literal::new("a")])]);
        assert_eq!(super::solve(cnf), true);
    }

    #[test]
    fn dpll_3() {
        let cnf = Cnf::new(vec![
            Clause::new(vec![Literal::new("a")]),
            Clause::new(vec![Literal::new("-a")]),
        ]);
        assert_eq!(super::solve(cnf), false);
    }

    #[test]
    fn dpll_4() {
        let cnf = Cnf::new(vec![Clause::new(vec![
            Literal::new("a"),
            Literal::new("b"),
        ])]);
        assert_eq!(super::solve(cnf), true);
    }

    #[test]
    fn dpll_5() {
        let cnf = Cnf::new(vec![
            Clause::new(vec![Literal::new("a"), Literal::new("-b")]),
            Clause::new(vec![Literal::new("-a"), Literal::new("b")]),
        ]);
        assert_eq!(super::solve(cnf), true);
    }

    #[test]
    fn dpll_6() {
        let cnf = Cnf::new(vec![
            Clause::new(vec![Literal::new("x1"), Literal::new("x2")]),
            Clause::new(vec![Literal::new("-x1"), Literal::new("x3")]),
            Clause::new(vec![Literal::new("x1"), Literal::new("-x3")]),
            Clause::new(vec![Literal::new("-x2"), Literal::new("-x3")]),
        ]);
        assert_eq!(super::solve(cnf), true);
    }
}
