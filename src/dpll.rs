use crate::types::Cnf;

pub fn solve(cnf: Cnf) -> bool {
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
}
