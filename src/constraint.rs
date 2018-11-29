use board::{Board, Item};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
    Div,
    Sub,
    Equal,
    Unique,
}

impl Op {
    fn from(s: &str) -> Op {
        use self::Op::*;
        match s {
            "*" => Mul,
            "+" => Add,
            "/" => Div,
            "-" => Sub,
            "=" => Equal,
            _ => Unique,
        }
    }
}

#[derive(Debug)]
pub struct Constraint {
    coords: Vec<(usize, usize)>,
    operation: Op,
    value: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConstraintResult {
    Okay,
    Violated,
    Solved,
    BadConstraint,
}

pub struct Constraints(Vec<Constraint>);

impl Constraints {
    pub fn new(size: usize) -> Self {
        let mut me = Constraints(Vec::with_capacity(2 * size * size));
        me.add_basic_constraints(size);
        me
    }

    fn add_basic_constraints(&mut self, size: usize) {
        for r in 0..size {
            let mut coords = Vec::with_capacity(size);
            for c in 0..size {
                coords.push((r, c));
            }
            self.0.push(Constraint {
                coords,
                operation: Op::Unique,
                value: None,
            });
        }
        for c in 0..size {
            let mut coords = Vec::with_capacity(size);
            for r in 0..size {
                coords.push((r, c));
            }
            self.0.push(Constraint {
                coords,
                operation: Op::Unique,
                value: None,
            });
        }
    }

    pub fn add(&mut self, line: String, puzzle: &[[char; 7]; 7]) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            panic!("Bad constraint format: {}", line);
        }
        let letter = parts[0].chars().next().unwrap();
        let coords = coords_for_char(puzzle, letter);
        let operation = Op::from(parts[1]);
        let value = parts[2].parse::<u32>().ok();
        self.0.push(Constraint {
            coords,
            operation,
            value,
        });
    }

    pub fn apply_equality(&self, result: &mut Board) {
        for constraint in self.0.iter() {
            match constraint.operation {
                Op::Equal => {
                    if constraint.coords.len() != 1 {
                        panic!("Bad equality constraint: too many coordinates!");
                    }
                    let (r, c) = constraint.coords[0];
                    if let Some(val) = constraint.value {
                        result.0[r][c] = Item::Solved(val);
                    } else {
                        panic!("Bad equality constraint: missing value!");
                    }
                }
                _ => {}
            }
        }
    }

    pub fn check(&self, result: &Board) -> ConstraintResult {
        use self::ConstraintResult::*;
        let mut okay = false;
        for constraint in self.0.iter() {
            match constraint.satisfied(result) {
                r @ Violated | r @ BadConstraint => return r,
                Solved => {}
                Okay => {
                    okay = true;
                }
            }
        }
        if okay {
            Okay
        } else {
            Solved
        }
    }
}

impl Constraint {
    fn satisfied(&self, ans: &Board) -> ConstraintResult {
        match self.operation {
            Op::Add => {
                let mut sum = 0;
                for (r, c) in self.coords.iter() {
                    let i = ans.0[*r][*c];
                    if i.is_hole() {
                        return ConstraintResult::Okay;
                    }
                    sum += i.value();
                }
                if let Some(val) = self.value {
                    if val == sum {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Mul => {
                let mut prod = 1;
                for (r, c) in self.coords.iter() {
                    let i = ans.0[*r][*c];
                    if i.is_hole() {
                        return ConstraintResult::Okay;
                    }
                    prod *= i.value();
                }
                if let Some(val) = self.value {
                    if val == prod {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Div => {
                if self.coords.len() != 2 {
                    return ConstraintResult::BadConstraint;
                }
                let (r1, c1) = self.coords[0];
                let (r2, c2) = self.coords[1];
                let i1 = ans.0[r1][c1];
                let i2 = ans.0[r2][c2];
                if i1.is_hole() || i2.is_hole() {
                    return ConstraintResult::Okay;
                }
                if let Some(val) = self.value {
                    if i1.value() / i2.value() == val || i2.value() / i1.value() == val {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Sub => {
                if self.coords.len() != 2 {
                    return ConstraintResult::BadConstraint;
                }
                let (r1, c1) = self.coords[0];
                let (r2, c2) = self.coords[1];
                let i1 = ans.0[r1][c1];
                let i2 = ans.0[r2][c2];
                if i1.is_hole() || i2.is_hole() {
                    return ConstraintResult::Okay;
                }
                if let Some(val) = self.value {
                    if i1.value() == val + i2.value() || i2.value() == val + i1.value() {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Equal => {
                if self.coords.len() != 1 {
                    return ConstraintResult::BadConstraint;
                }
                let i = ans.0[self.coords[0].0][self.coords[0].1];
                if i.is_hole() {
                    return ConstraintResult::Okay;
                }
                if let Some(val) = self.value {
                    if val == i.value() {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Unique => {
                let mut seen = [false; 7];
                let mut seen_hole = false;
                for (r, c) in self.coords.iter() {
                    let item = ans.0[*r][*c];
                    if item.is_hole() {
                        seen_hole = true;
                        continue;
                    }
                    if seen[(item.value() - 1) as usize] {
                        return ConstraintResult::Violated;
                    }
                    seen[(item.value() - 1) as usize] = true;
                }
                if seen_hole {
                    ConstraintResult::Okay
                } else {
                    ConstraintResult::Solved
                }
            }
        }
    }
}

fn coords_for_char(puzzle: &[[char; 7]; 7], val: char) -> Vec<(usize, usize)> {
    let mut result = Vec::with_capacity(5);
    for r in 0..7 {
        for c in 0..7 {
            if puzzle[r][c] == val {
                result.push((r, c));
            }
        }
    }
    result
}
