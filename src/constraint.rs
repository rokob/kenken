use board::Board;
use puzzle::Puzzle;
use MAX_SIZE;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Mul,
    Div,
    Sub,
    Equal,
    Latin,
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
            "B" => Latin,
            _ => panic!("Unknown operation"),
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

pub struct Constraints(Vec<Constraint>, usize);

impl Constraints {
    pub fn new(size: usize) -> Self {
        Constraints(Vec::with_capacity(2 * size * size), size)
    }

    pub fn add(&mut self, line: &str, puzzle: &Puzzle) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 3 {
            panic!("Bad constraint format: {}", line);
        }
        let letter = parts[0].chars().next().unwrap();
        let coords = puzzle.coords_for_char(letter);
        let operation = Op::from(parts[1]);
        let value = parts[2].parse::<u32>().ok();
        if Op::Latin == operation {
            self.generate_latin_constraints(
                value.expect("box constraint must have a value") as usize
            );
        } else {
            self.0.push(Constraint {
                coords,
                operation,
                value,
            });
        }
    }

    pub fn apply_equality(&self, result: &mut Board) {
        for constraint in self.0.iter() {
            if let Op::Equal = constraint.operation {
                if constraint.coords.len() != 1 {
                    panic!("Bad equality constraint: too many coordinates!");
                }
                let (r, c) = constraint.coords[0];
                if let Some(val) = constraint.value {
                    result.solve(r, c, val);
                } else {
                    panic!("Bad equality constraint: missing value!");
                }
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

    fn generate_latin_constraints(&mut self, size: usize) {
        let count = self.1 / size;
        for box_row in 0..count {
            for box_col in 0..count {
                let origin = (box_row * size, box_col * size);
                let mut coords = Vec::with_capacity(size * size);
                for r in 0..size {
                    for c in 0..size {
                        coords.push((origin.0 + r, origin.1 + c));
                    }
                }
                self.0.push(Constraint {
                    coords,
                    operation: Op::Latin,
                    value: Some(size as u32),
                });
            }
        }
    }
}

impl Constraint {
    fn satisfied(&self, ans: &Board) -> ConstraintResult {
        match self.operation {
            Op::Add => {
                let mut sum = 0;
                let mut seen_hole = false;
                for (r, c) in self.coords.iter() {
                    if ans.is_hole(*r, *c) {
                        seen_hole = true;
                        continue;
                    }
                    sum += ans.value(*r, *c);
                }
                if let Some(val) = self.value {
                    if seen_hole {
                        if sum < val {
                            return ConstraintResult::Okay;
                        } else {
                            return ConstraintResult::Violated;
                        }
                    }
                    if val == sum {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Mul => {
                let mut prod = 1;
                let mut seen_hole = false;
                for (r, c) in self.coords.iter() {
                    if ans.is_hole(*r, *c) {
                        seen_hole = true;
                        continue;
                    }
                    prod *= ans.value(*r, *c);
                }
                if let Some(val) = self.value {
                    if seen_hole {
                        if prod <= val && val % prod == 0 {
                            return ConstraintResult::Okay;
                        } else {
                            return ConstraintResult::Violated;
                        }
                    }
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
                let a = self.coords[0];
                let b = self.coords[1];
                if let Some(val) = self.value {
                    ans.could_div_equal(a, b, val)
                } else {
                    ConstraintResult::Violated
                }
            }
            Op::Sub => {
                if self.coords.len() != 2 {
                    return ConstraintResult::BadConstraint;
                }
                let a = self.coords[0];
                let b = self.coords[1];
                if let Some(val) = self.value {
                    ans.could_sub_equal(a, b, val)
                } else {
                    ConstraintResult::Violated
                }
            }
            Op::Equal => {
                if self.coords.len() != 1 {
                    return ConstraintResult::BadConstraint;
                }
                let (r, c) = (self.coords[0].0, self.coords[0].1);
                if ans.is_hole(r, c) {
                    return ConstraintResult::Okay;
                }
                if let Some(val) = self.value {
                    if val == ans.value(r, c) {
                        return ConstraintResult::Solved;
                    }
                }
                ConstraintResult::Violated
            }
            Op::Latin => {
                let mut seen = [false; MAX_SIZE];
                let mut seen_hole = false;
                for (r, c) in self.coords.iter() {
                    if ans.is_hole(*r, *c) {
                        seen_hole = true;
                        continue;
                    }
                    if seen[(ans.value(*r, *c) - 1) as usize] {
                        return ConstraintResult::Violated;
                    }
                    seen[(ans.value(*r, *c) - 1) as usize] = true;
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
