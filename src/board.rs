use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Hole,
    Guess(u32),
    Solved(u32),
}

impl Item {
    pub fn is_hole(&self) -> bool {
        match self {
            Item::Hole => true,
            _ => false,
        }
    }

    pub fn is_solved(&self) -> bool {
        match self {
            Item::Solved(_) => true,
            _ => false,
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Item::Guess(x) | Item::Solved(x) => *x,
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

pub struct Board(pub [[Item; 7]; 7], pub usize);

impl Board {
    pub fn new(size: usize) -> Self {
        Board([[Item::Hole; 7]; 7], size)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.1 - 1 {
            for c in 0..self.1 - 1 {
                write!(f, "{} ", self.0[r][c])?
            }
            write!(f, "{}\n", self.0[r][self.1 - 1])?
        }
        for c in 0..self.1 - 1 {
            write!(f, "{} ", self.0[self.1 - 1][c])?
        }
        write!(f, "{}", self.0[self.1 - 1][self.1 - 1])
    }
}
