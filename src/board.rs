use std::fmt;
use MAX_SIZE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Hole,
    Guess(u32),
    Solved(u32),
}

impl Item {
    pub fn is_hole(self) -> bool {
        match self {
            Item::Hole => true,
            _ => false,
        }
    }

    pub fn is_solved(self) -> bool {
        match self {
            Item::Solved(_) => true,
            _ => false,
        }
    }

    pub fn value(self) -> u32 {
        match self {
            Item::Guess(x) | Item::Solved(x) => x,
            Item::Hole => 0,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Item::Hole => write!(f, "."),
            Item::Guess(x) | Item::Solved(x) => write!(f, "{}", x),
        }
    }
}

pub struct Board(pub [[Item; MAX_SIZE]; MAX_SIZE], pub usize);

impl Board {
    pub fn new(size: usize) -> Self {
        Board([[Item::Hole; MAX_SIZE]; MAX_SIZE], size)
    }

    pub fn solve(&mut self, r: usize, c: usize, val: u32) {
        self.0[r][c] = Item::Solved(val);
    }

    pub fn is_hole(&self, r: usize, c: usize) -> bool {
        self.0[r][c].is_hole()
    }

    pub fn value(&self, r: usize, c: usize) -> u32 {
        self.0[r][c].value()
    }

    pub fn either_hole(&self, a: (usize, usize), b: (usize, usize)) -> bool {
        self.0[a.0][a.1].is_hole() || self.0[b.0][b.1].is_hole()
    }

    pub fn div_equal(&self, a: (usize, usize), b: (usize, usize), val: u32) -> bool {
        let a_val = self.0[a.0][a.1].value();
        let b_val = self.0[b.0][b.1].value();
        a_val == val * b_val || b_val == val * a_val
    }

    pub fn sub_equal(&self, a: (usize, usize), b: (usize, usize), val: u32) -> bool {
        let a_val = self.0[a.0][a.1].value();
        let b_val = self.0[b.0][b.1].value();
        a_val == val + b_val || b_val == val + a_val
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.1 - 1 {
            for c in 0..self.1 - 1 {
                write!(f, "{} ", self.0[r][c])?
            }
            writeln!(f, "{}", self.0[r][self.1 - 1])?
        }
        for c in 0..self.1 - 1 {
            write!(f, "{} ", self.0[self.1 - 1][c])?
        }
        write!(f, "{}", self.0[self.1 - 1][self.1 - 1])
    }
}
