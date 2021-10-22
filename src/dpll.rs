use crate::types::*;

#[derive(Debug, PartialEq, Eq)]
pub enum SatResult {
    Sat(Solution),
    Unsat,
}

impl SatResult {
    pub fn is_sat(&self) -> bool {
        match self {
            SatResult::Sat(_) => true,
            &SatResult::Unsat => false,
        }
    }

    pub fn is_unsat(&self) -> bool {
        match self {
            SatResult::Sat(_) => false,
            &SatResult::Unsat => true,
        }
    }
}

pub fn solve(mut cnf: Cnf) -> SatResult {
    apply_unit_rule(&mut cnf);

    if cnf.is_consistent() {
        return SatResult::Sat(cnf.determined);
    }
    if cnf.has_empty_clause() {
        return SatResult::Unsat;
    }

    apply_splitting_rule(&mut cnf)
}

fn apply_unit_rule(cnf: &mut Cnf) {
    for literal in cnf.find_unit_clauses() {
        cnf.remove_clauses_which_has(&literal);
        cnf.remove_from_all(&literal.inverted());
        cnf.determine(literal.var, !literal.inverted);
    }
}

fn apply_splitting_rule(cnf: &mut Cnf) -> SatResult {
    let literal = match cnf.head_literal() {
        Some(l) => l,
        None => return SatResult::Unsat,
    };

    let mut cnf_true = cnf.clone();
    cnf_true.add_clause(&[literal.clone()]);
    if let SatResult::Sat(solution) = solve(cnf_true) {
        return SatResult::Sat(solution);
    }

    let mut cnf_false = cnf.clone();
    cnf_false.add_clause(&[literal.inverted()]);
    if let SatResult::Sat(solution) = solve(cnf_false) {
        return SatResult::Sat(solution);
    }

    SatResult::Unsat
}

#[cfg(test)]
mod tests {
    use crate::dimacs;

    #[test]
    fn dpll_1() {
        let input = "p cnf 0 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_sat());
    }

    #[test]
    fn dpll_2() {
        let input = "p cnf 1 1
                        1 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_sat());
    }

    #[test]
    fn dpll_3() {
        let input = "p cnf 1 2
                        1 0
                        -1 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_unsat());
    }

    #[test]
    fn dpll_4() {
        let input = "p cnf 2 1
                        1 2 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_sat());
    }

    #[test]
    fn dpll_5() {
        let input = "p cnf 2 2
                        1 -2 0
                        -1 2 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_sat());
    }

    #[test]
    fn dpll_6() {
        let input = "p cnf 3 4
                        1 2 0
                        -1 3 0
                        -1 -3 0
                        -2 -3 0";
        let cnf = dimacs::parse(input);
        assert!(super::solve(cnf).is_sat());
    }
}
