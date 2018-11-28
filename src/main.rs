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

fn main() {
    pretty_env_logger::init();

    trace!("loading puzzle.dat");
    let filename = "puzzle.dat";
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let size = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut constraints = Constraints::new(size);

    let mut puzzle = [[' '; 7]; 7];
    for (r, line) in lines.enumerate() {
        let line = line.unwrap();
        if r < size {
            process_puzzle(line, r, &mut puzzle);
        } else {
            constraints.add(line, &puzzle);
        }
    }

    let result = solve(size, constraints);
    warn!("Solved:");
    println!("{}", result);
}

fn process_puzzle(line: String, r: usize, puzzle: &mut [[char; 7]; 7]) {
    for (c, val) in line.chars().enumerate() {
        puzzle[r][c] = val;
    }
}
