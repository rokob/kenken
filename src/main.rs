extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod board;
mod constraint;
mod solver;

use constraint::Constraints;
use solver::solve;

use std::env;

fn main() {
    pretty_env_logger::init();

    let args = env::args().collect::<Vec<_>>();
    let filename = if args.len() == 1 {
        "puzzle.dat".to_owned()
    } else {
        args[1].clone()
    };
    warn!("loading {}", filename);
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let size = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut constraints = Constraints::new(size);

    let mut puzzle = [[' '; 7]; 7];
    for (r, line) in lines.enumerate() {
        let line = line.unwrap();
        if r < size {
            for (c, val) in line.chars().enumerate() {
                puzzle[r][c] = val;
            }
        } else {
            constraints.add(line, &puzzle);
        }
    }

    let result = solve(size, constraints);
    println!("{}", result);
}
