use crate::types::*;

pub fn parse(input: &str) -> Cnf {
    let mut tokens = input.split_ascii_whitespace().peekable();
    while tokens.peek().unwrap() != &"p" {
        tokens.next().unwrap();
    }

    if tokens.next().unwrap() != "p" || tokens.next().unwrap() != "cnf" {
        panic!("requires DIMACS format");
    }

    let num_of_variables = tokens.next().unwrap().parse::<usize>().unwrap();
    let num_of_clauses = tokens.next().unwrap().parse::<usize>().unwrap();

    let mut cnf = Cnf::new(num_of_variables);
    for _ in 0..num_of_clauses {
        let mut literals = Vec::new();

        loop {
            let token = tokens.next().unwrap();
            let var = token.parse::<isize>().unwrap();
            if var == 0 {
                break;
            }

            literals.push(parse_literal(var));
        }

        cnf.add_clause(&literals);
    }

    cnf
}

fn parse_literal(var: isize) -> Literal {
    if var > 0 {
        Literal::new(var as usize, false)
    } else {
        Literal::new(-var as usize, true)
    }
}
