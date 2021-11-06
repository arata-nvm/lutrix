use std::env;

use lutrix::{
    parser::dimacs,
    sat::{types::Variable, Solver},
};

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

    let mut solver = Solver::new();
    solver.set_formula(formula);
    match solver.check() {
        true => {
            println!("SAT");

            let model = solver.model();
            let mut vars = model.keys().collect::<Vec<&Variable>>();
            vars.sort();
            for var in vars {
                println!("x{} = {}", var, model[var]);
            }
        }
        false => {
            println!("UNSAT");
        }
    }
}
