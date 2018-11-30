#[macro_use]
extern crate log;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod board;
mod constraint;
mod puzzle;
pub mod solver;

use constraint::Constraints;
use puzzle::Puzzle;

const MAX_SIZE: usize = 9;

pub fn solve(filename: &str) -> board::Board {
    let (size, constraints) = get_input(filename);
    solver::solve(size, &constraints)
}

pub fn get_input(filename: &str) -> (usize, Constraints) {
    info!("loading {}", filename);
    let f = File::open(filename).unwrap();
    let f = BufReader::new(f);
    let mut lines = f.lines();

    let size = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut constraints = Constraints::new(size);

    let mut puzzle = Puzzle::default();
    for (r, line) in lines.enumerate() {
        let line = line.unwrap();
        if r < size {
            for (c, val) in line.chars().enumerate() {
                puzzle.set(r, c, val);
            }
        } else {
            constraints.add(&line, &puzzle);
        }
    }
    (size, constraints)
}
