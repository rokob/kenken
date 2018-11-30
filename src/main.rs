extern crate kenken;
extern crate pretty_env_logger;

use std::env;

fn main() {
    pretty_env_logger::init();

    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() == 1 {
        "puzzle.dat".to_owned()
    } else {
        args[1].clone()
    };

    let result = kenken::solve(&filename);
    println!("{}", result);
}
