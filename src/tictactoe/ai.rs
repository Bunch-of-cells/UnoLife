use super::{Board, Mark};

pub const INFINITY: i32 = i32::MAX;

pub fn get_best_move(board: &mut Board) -> Option<(usize, usize)> {
    let mut best_score = INFINITY;
    let mut best_move = None;
    for (x, y) in generate_moves(board) {
        let mut board = board.clone();
        board.mov(x, y).unwrap();
        let score = negamax(&mut board, 30, -INFINITY, INFINITY);
        if score < best_score {
            best_score = score;
            best_move = Some((x, y));
        }
    }
    best_move
}

pub fn evaluate(board: &Board, depth: i32) -> i32 {
    if board.is_over() == Mark::X {
        if board.turn == Mark::X {
            return INFINITY-depth;
        } else {
            return -INFINITY+depth;
        }
    } else if board.is_over() == Mark::O {
        if board.turn == Mark::X {
            return -INFINITY+depth;
        } else {
            return INFINITY-depth;
        }
    }
    return 0;
}

pub fn generate_moves(board: &Board) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    for x in 0..3 {
        for y in 0..3 {
            if board.cells[x][y].is_none() {
                moves.push((x, y));
            }
        }
    }
    moves
}

pub fn negamax(board: &mut Board, depth: i32, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return evaluate(board, depth);
    }
    let moves = generate_moves(board);
    if moves.len() == 0 {
        return evaluate(board, depth);
    }
    let mut best_move = -INFINITY;
    for move_ in moves.iter() {
        let x = move_.0;
        let y = move_.1;
        if board.mov(x, y) == Ok(Mark::None) {
            return 0;
        } 
        let score = -negamax(board, depth - 1, -beta, -alpha);
        if score == INFINITY || score == -INFINITY {
            return score;
        }
        board.cells[x][y] = None;
        board.turn = match board.turn {
            Mark::X => Mark::O,
            Mark::O => Mark::X,
            _ => Mark::None,
        };
        if score > best_move {
            best_move = score;
        }
        if score > alpha {
            alpha = score;
            if score >= beta {
                return beta;
            }
        }
    }
    return alpha;
}