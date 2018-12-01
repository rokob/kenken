use constraint::ConstraintResult;
use std::fmt;
use MAX_SIZE;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Backtrack {
    Solved,
    Reset,
    Next,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Item {
    Hole,
    Guess(u32),
    Solved(u32),
}

impl Item {
    #[inline]
    pub fn is_hole(self) -> bool {
        self == Item::Hole
    }

    pub fn is_solved(self) -> bool {
        match self {
            Item::Solved(_) => true,
            _ => false,
        }
    }

    #[inline]
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

macro_rules! mget {
    ($m:tt, $x:expr, $y:expr) => {
        $m[$x + $y * MAX_SIZE]
    };
}

pub struct Board {
    board: [Item; MAX_SIZE * MAX_SIZE],
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Board {
            board: [Item::Hole; MAX_SIZE * MAX_SIZE],
            size,
        }
    }

    pub fn solve(&mut self, r: usize, c: usize, val: u32) {
        mget!((self.board), r, c) = Item::Solved(val);
    }

    pub fn is_hole(&self, r: usize, c: usize) -> bool {
        mget!((self.board), r, c).is_hole()
    }

    pub fn is_solved(&self, r: usize, c: usize) -> bool {
        mget!((self.board), r, c).is_solved()
    }

    pub fn value(&self, r: usize, c: usize) -> u32 {
        mget!((self.board), r, c).value()
    }

    pub fn could_div_equal(
        &self,
        a: (usize, usize),
        b: (usize, usize),
        val: u32,
    ) -> ConstraintResult {
        let a_item = mget!((self.board), a.0, a.1);
        let b_item = mget!((self.board), b.0, b.1);
        let a_hole = a_item.is_hole();
        let b_hole = b_item.is_hole();
        if a_hole && b_hole {
            return ConstraintResult::Okay;
        }
        if a_hole {
            let b_val = b_item.value();
            if val * b_val <= self.size as u32 || b_val % val == 0 {
                return ConstraintResult::Okay;
            } else {
                return ConstraintResult::Violated;
            }
        }
        if b_hole {
            let a_val = a_item.value();
            if val * a_val <= self.size as u32 || a_val % val == 0 {
                return ConstraintResult::Okay;
            } else {
                return ConstraintResult::Violated;
            }
        }
        let a_val = a_item.value();
        let b_val = b_item.value();
        if a_val == val * b_val || b_val == val * a_val {
            ConstraintResult::Solved
        } else {
            ConstraintResult::Violated
        }
    }

    pub fn could_sub_equal(
        &self,
        a: (usize, usize),
        b: (usize, usize),
        val: u32,
    ) -> ConstraintResult {
        let a_item = mget!((self.board), a.0, a.1);
        let b_item = mget!((self.board), b.0, b.1);
        if a_item.is_hole() && b_item.is_hole() {
            return ConstraintResult::Okay;
        }
        if a_item.is_hole() {
            let b_val = b_item.value();
            if val + b_val <= self.size as u32 || b_val > val {
                return ConstraintResult::Okay;
            } else {
                return ConstraintResult::Violated;
            }
        }
        if b_item.is_hole() {
            let a_val = a_item.value();
            if val + a_val <= self.size as u32 || a_val > val {
                return ConstraintResult::Okay;
            } else {
                return ConstraintResult::Violated;
            }
        }
        let a_val = a_item.value();
        let b_val = b_item.value();
        if a_val == val + b_val || b_val == val + a_val {
            ConstraintResult::Solved
        } else {
            ConstraintResult::Violated
        }
    }

    pub fn backtrack(&mut self, pos: &(usize, usize)) -> Backtrack {
        if mget!((self.board), pos.0, pos.1).is_hole() {
            return Backtrack::Reset;
        }
        if mget!((self.board), pos.0, pos.1).is_solved() {
            return Backtrack::Solved;
        }
        if let Item::Guess(x) = mget!((self.board), pos.0, pos.1) {
            if x == self.size as u32 {
                mget!((self.board), pos.0, pos.1) = Item::Hole;
                Backtrack::Reset
            } else {
                let poss = self.get_possible(pos);
                if let Some(i) = poss[1..=self.size].iter().skip(x as usize).position(|&b| b) {
                    mget!((self.board), pos.0, pos.1) = Item::Guess(x + 1 + i as u32);
                    return Backtrack::Next;
                }
                mget!((self.board), pos.0, pos.1) = Item::Hole;
                Backtrack::Reset
            }
        } else {
            panic!("Bad backtracking cell @ ({}, {})", pos.0, pos.1);
        }
    }

    pub fn initial(&mut self, pos: &(usize, usize)) -> Backtrack {
        let poss = self.get_possible(pos);
        if let Some(i) = poss[1..=self.size].iter().position(|&b| b) {
            mget!((self.board), pos.0, pos.1) = Item::Guess(1 + i as u32);
            Backtrack::Next
        } else {
            Backtrack::Reset
        }
    }

    fn get_possible(&self, pos: &(usize, usize)) -> [bool; MAX_SIZE + 1] {
        let mut available = [true; MAX_SIZE + 1];
        for x in 0..self.size {
            if x != pos.0 {
                available[mget!((self.board), x, pos.1).value() as usize] = false;
            }
            if x != pos.1 {
                available[mget!((self.board), pos.0, x).value() as usize] = false;
            }
        }
        available
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.size - 1 {
            for c in 0..self.size - 1 {
                write!(f, "{} ", mget!((self.board), r, c))?
            }
            writeln!(f, "{}", mget!((self.board), r, self.size - 1))?
        }
        for c in 0..self.size - 1 {
            write!(f, "{} ", mget!((self.board), self.size - 1, c))?
        }
        write!(f, "{}", mget!((self.board), self.size - 1, self.size - 1))
    }
}
