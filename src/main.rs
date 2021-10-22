use std::env;

use lutrix::{dimacs, dpll, types::Variable};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("usage: lutrix <input-file>");
        return;
    }

    println!("[*] reading file: {}", args[1]);
    let input = std::fs::read_to_string(&args[1]).expect("cannot read file");
    let formula = dimacs::parse(&input);
    println!("[*] formula = {}", formula);
    match dpll::solve(formula) {
        dpll::SatResult::Sat(solution) => {
            println!("SAT");

            let mut vars = solution.keys().collect::<Vec<&Variable>>();
            vars.sort();
            for var in vars {
                println!("x{} = {}", var, solution[var]);
            }
        }
        dpll::SatResult::Unsat => {
            println!("UNSAT");
        }
    }
}
