use board::{Board, Item};
use constraint::{ConstraintResult, Constraints};

pub fn solve(size: usize, mut constraints: Constraints) -> Board {
    use self::ConstraintResult::*;
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
            Solved => break,
            Violated => backtrack(size, &mut result, &mut pos),
            Okay => next(size, &mut result, &mut pos),
            BadConstraint => break,
        }
        if log_enabled!(log::Level::Debug) {
            if steps % 50 == 0 {
                debug!("Step {}", steps);
                debug!("\n{}", result);
            }
        } else if log_enabled!(log::Level::Info) {
            if steps % 100 == 0 {
                info!("Step {}", steps);
                info!("\n{}", result);
            }
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
                let prev_pos = prev_position(size, cur);
                *cur = prev_pos;
                continue;
            }
            Item::Guess(x) => {
                if x as usize == size {
                    board.0[cur.0][cur.1] = Item::Hole;
                    let prev_pos = prev_position(size, cur);
                    *cur = prev_pos;
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
        let next_pos = next_position(size, cur);
        if board.0[next_pos.0][next_pos.1].is_hole() {
            board.0[next_pos.0][next_pos.1] = Item::Guess(1);
            *cur = next_pos;
            break;
        } else if board.0[next_pos.0][next_pos.1].is_solved() {
            *cur = next_pos;
            continue;
        } else {
            panic!("This shouldn't happen @ ({}, {})", next_pos.0, next_pos.1);
        }
    }
}

fn prev_position(size: usize, cur: &(usize, usize)) -> (usize, usize) {
    if cur.0 == 0 {
        (size - 1, cur.1 - 1)
    } else {
        (cur.0 - 1, cur.1)
    }
}

fn next_position(size: usize, cur: &(usize, usize)) -> (usize, usize) {
    if cur.0 + 1 == size {
        (0, cur.1 + 1)
    } else {
        (cur.0 + 1, cur.1)
    }
}
