use board::{Board, Item};
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
        if log_enabled!(log::Level::Debug) {
            if steps % 50 == 0 {
                debug!("Step {}", steps);
                debug!("\n{}", result);
            }
        } else if log_enabled!(log::Level::Info) && steps % 100 == 0 {
            info!("Step {}", steps);
            info!("\n{}", result);
        }
    }
    warn!("Done @ {}", steps);
    result
}

fn backtrack(size: usize, board: &mut Board, cur: &mut (usize, usize)) {
    trace!("backtracking...");
    loop {
        match board.0[cur.0][cur.1] {
            Item::Hole => panic!(
                "Backtracking from a hole makes no sense @ ({}, {})",
                cur.0, cur.1
            ),
            Item::Solved(_) => {
                prev_position(size, cur);
                continue;
            }
            Item::Guess(x) => {
                if x as usize == size {
                    board.0[cur.0][cur.1] = Item::Hole;
                    prev_position(size, cur);
                    continue;
                } else {
                    board.0[cur.0][cur.1] = Item::Guess(x + 1);
                    break;
                }
            }
        }
    }
}

fn next(size: usize, board: &mut Board, cur: &mut (usize, usize)) {
    if board.0[cur.0][cur.1].is_hole() {
        board.0[cur.0][cur.1] = Item::Guess(1);
        return;
    }
    loop {
        next_position(size, cur);
        if board.0[cur.0][cur.1].is_hole() {
            board.0[cur.0][cur.1] = Item::Guess(1);
            break;
        } else if board.0[cur.0][cur.1].is_solved() {
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
