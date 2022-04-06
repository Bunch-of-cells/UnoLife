pub mod ai;
pub mod board;
pub mod ui;

pub use ai::*;
pub use board::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_over_1() {
        /*
        O O -
        - O -
        X X X
         */
        let board = Board {
            cells: [
                [Mark::O, Mark::O, Mark::None],
                [Mark::None, Mark::O, Mark::None],
                [Mark::X, Mark::X, Mark::X],
            ],
            turn: Mark::X,
        };
        assert_eq!(board.is_over(), Mark::X);
    }

    #[test]
    fn test_game_over_2() {
        /*
        O X -
        - O -
        X X O
         */
        let board = Board {
            cells: [
                [Mark::O, Mark::X, Mark::None],
                [Mark::None, Mark::O, Mark::None],
                [Mark::X, Mark::X, Mark::O],
            ],
            turn: Mark::X,
        };
        assert_eq!(board.is_over(), Mark::O);
    }

    #[test]
    fn test_game_over_3() {
        /*
        O - -
        - O -
        X X -
         */
        let board = Board {
            cells: [
                [Mark::O, Mark::None, Mark::None],
                [Mark::None, Mark::O, Mark::None],
                [Mark::X, Mark::X, Mark::None],
            ],
            turn: Mark::X,
        };
        assert_eq!(board.is_over(), Mark::None);
    }

    #[test]
    fn test_negamax_tactic_1() {
        /*
        O X -
        - O -
        ! X X

        O at bottom left (!) wins
         */
        let mut board = Board {
            cells: [
                [Mark::O, Mark::X, Mark::None],
                [Mark::None, Mark::O, Mark::None],
                [Mark::None, Mark::X, Mark::X],
            ],
            turn: Mark::O,
        };
        let bm = negamax_root(&mut board);
        assert_eq!(bm.0, 2);
        assert_eq!(bm.1, 0);
    }
}
