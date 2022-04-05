use super::{Board, Mark};
use std::cmp;

pub fn get_best_move(board: &Board) -> Option<(usize, usize)> {
    let mut board_new = board.clone();
    let mut best_score = 1;
    let mut best_move = None;
    for x in 0..3 {
        for y in 0..3 {
            if board_new.cells[x][y].is_none() {
                board_new.mov(x, y).unwrap();
                let score = -negamax(&mut board_new, -1, 1, board.turn as u8 != 0);
                board_new.cells[x][y] = None;
                board_new.turn = board.turn;
                if score > best_score {
                    best_score = score;
                    best_move = Some((x, y));
                }
            }
        }
    }
    best_move
}

fn negamax(board: &mut Board, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    if let Some(winner) = board.is_over() {
        if winner == Mark::X {
            return 1;
        } else if winner == Mark::O {
            return -1;
        }
    }
    let mut best_score = if maximizing { -1 } else { 1 };
    for x in 0..3 {
        for y in 0..3 {
            if board.cells[x][y].is_none() {
                board.cells[x][y] = Some(if maximizing { Mark::X } else { Mark::O });
                let score = -negamax(board, -beta, -alpha, !maximizing);
                board.cells[x][y] = None;
                board.turn = match board.turn {
                    Mark::X => Mark::O,
                    Mark::O => Mark::X,
                };
                if maximizing {
                    if score > best_score {
                        best_score = score;
                    }
                    alpha = cmp::max(alpha, score);
                } else {
                    if score < best_score {
                        best_score = score;
                    }
                    beta = cmp::min(beta, score);
                }
                if beta <= alpha {
                    break;
                }
            }
        }
    }
    best_score
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ai_works() {
        let board = Board::new();
        println!("{:?}", get_best_move(&board));
        // assert_eq!(ai.play(), None);
    }
}
