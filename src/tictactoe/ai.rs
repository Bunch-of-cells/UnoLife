use super::{Board, Mark};

pub const INFINITY: i32 = i32::MAX - 100;

pub fn evaluate(board: &Board, ply: i32) -> i32 {
    let res = board.is_over();
    if res == Mark::X {
        if board.turn == Mark::X {
            INFINITY - ply
        } else {
            -INFINITY + ply
        }
    } else if res == Mark::O {
        if board.turn == Mark::X {
            -INFINITY + ply
        } else {
            INFINITY - ply
        }
    } else {
        0
    }
}

pub fn generate_moves(board: &Board) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    if board.is_over() != Mark::None {
        return moves;
    }
    for x in 0..3 {
        for y in 0..3 {
            if board.cells[x][y] == Mark::None {
                moves.push((x, y));
            }
        }
    }
    moves
}

pub fn negamax_root(board: &mut Board) -> (usize, usize) {
    let moves = generate_moves(board);
    if moves.is_empty() {
        // Should never happen
        return (0, 0);
    }
    let mut best_score = -INFINITY;
    let mut best_move: (usize, usize) = (0, 0);

    let mut alpha = -INFINITY;
    let beta = INFINITY;

    for move_ in moves.iter() {
        let x = move_.0;
        let y = move_.1;

        board.make_move(x, y);
        let score = -negamax(board, -beta, -alpha, 1);
        board.undo_move(x, y);

        if score > best_score {
            best_score = score;
            best_move = *move_;
        }
        if score > alpha {
            alpha = score;
            if score >= beta {
                break;
            }
        }
    }
    best_move
}

pub fn negamax(board: &mut Board, mut alpha: i32, beta: i32, ply: i32) -> i32 {
    let moves = generate_moves(board);
    if moves.is_empty() {
        return evaluate(board, ply);
    }

    let mut best_score = -INFINITY;
    for move_ in moves.iter() {
        let x = move_.0;
        let y = move_.1;

        let e = board.make_move(x, y);
        debug_assert!(e.is_none());
        let score = -negamax(board, -beta, -alpha, ply + 1);
        board.undo_move(x, y);

        if score > best_score {
            best_score = score;
        }
        if score > alpha {
            alpha = score;
            if score >= beta {
                return beta;
            }
        }
    }
    alpha
}
