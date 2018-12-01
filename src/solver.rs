use board::{Backtrack, Board};
use constraint::{ConstraintResult, Constraints};

pub fn solve(size: usize, constraints: &Constraints) -> Board {
    let mut result = Board::new(size);
    constraints.apply_equality(&mut result);

    let mut pos = (0, 0);
    let mut steps = 0;
    loop {
        steps += 1;
        if steps < 10 {
            trace!("{} ({}, {})", steps, pos.0, pos.1);
            trace!("\n{}", result);
        }
        match constraints.check(&result) {
            ConstraintResult::Solved | ConstraintResult::BadConstraint => break,
            ConstraintResult::Violated => backtrack(size, &mut result, &mut pos),
            ConstraintResult::Okay => next(size, &mut result, &mut pos),
        }
        if log_enabled!(log::Level::Debug) && steps % 50 == 0 {
            debug!("Step {}", steps);
            debug!("\n{}", result);
        }
    }
    info!("Done @ {}", steps);
    result
}

fn backtrack(size: usize, board: &mut Board, cur: &mut (usize, usize)) {
    trace!("backtracking...");
    loop {
        match board.backtrack(cur) {
            Backtrack::Solved | Backtrack::Reset => {
                prev_position(size, cur);
                continue;
            }
            Backtrack::Next => {
                break;
            }
        }
    }
}

fn next(size: usize, board: &mut Board, cur: &mut (usize, usize)) {
    if board.is_hole(cur.0, cur.1) {
        board.initial(cur);
        return;
    }
    loop {
        next_position(size, cur);
        if board.is_hole(cur.0, cur.1) {
            board.initial(cur);
            break;
        } else if board.is_solved(cur.0, cur.1) {
            continue;
        } else {
            panic!("This shouldn't happen @ ({}, {})", cur.0, cur.1);
        }
    }
}

#[inline]
fn prev_position(size: usize, cur: &mut (usize, usize)) {
    *cur = if cur.0 == 0 {
        (size - 1, cur.1 - 1)
    } else {
        (cur.0 - 1, cur.1)
    }
}

#[inline]
fn next_position(size: usize, cur: &mut (usize, usize)) {
    *cur = if cur.0 + 1 == size {
        (0, cur.1 + 1)
    } else {
        (cur.0 + 1, cur.1)
    }
}
