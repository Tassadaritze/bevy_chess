#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum ChessPieceColour {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChessPieceKind {
    Pawn,
    Knight,
    Rook,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub struct ChessPiece {
    pub x: u32,
    pub y: u32,
    pub colour: ChessPieceColour,
    pub kind: ChessPieceKind,
    pub has_moved: bool,
}

impl ChessPiece {
    pub fn new(x: u32, y: u32, colour: ChessPieceColour, kind: ChessPieceKind) -> Self {
        Self {
            x,
            y,
            colour,
            kind,
            has_moved: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::board::Board;
    use crate::game::pieces::{ChessPiece, ChessPieceColour, ChessPieceKind};
    use crate::game::{Move, MoveFromTo};

    #[test]
    fn pawn_basic_moves() {
        let mut board = Board::new();
        // this is the basic pawn movement test, so we'll assume that all of our pawns have already moved
        board.board[7 - 3][1] = Some(ChessPiece {
            x: 1,
            y: 3,
            colour: ChessPieceColour::White,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 3][2] = Some(ChessPiece {
            x: 2,
            y: 3,
            colour: ChessPieceColour::White,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 4][2] = Some(ChessPiece {
            x: 2,
            y: 4,
            colour: ChessPieceColour::Black,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 4][3] = Some(ChessPiece {
            x: 3,
            y: 4,
            colour: ChessPieceColour::Black,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        // board.set(1, 3, ChessPieceColour::White, ChessPieceKind::Pawn);
        // board.set(2, 3, ChessPieceColour::White, ChessPieceKind::Pawn);
        // board.set(2, 4, ChessPieceColour::Black, ChessPieceKind::Pawn);
        // board.set(3, 4, ChessPieceColour::Black, ChessPieceKind::Pawn);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 1,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 4,
                    takes: true
                }
            ])),
            board.get_moves(1, 3)
        );
        assert_eq!(
            Some(HashSet::from([Move {
                x: 3,
                y: 4,
                takes: true
            }])),
            board.get_moves(2, 3)
        );
        assert_eq!(
            Some(HashSet::from([Move {
                x: 1,
                y: 3,
                takes: true
            }])),
            board.get_moves(2, 4)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 3,
                    takes: true
                }
            ])),
            board.get_moves(3, 4)
        );
    }

    #[test]
    fn knight_moves() {
        let mut board = Board::new();
        board.set(0, 0, ChessPieceColour::White, ChessPieceKind::Knight);
        board.set(1, 2, ChessPieceColour::Black, ChessPieceKind::Knight);
        board.set(0, 4, ChessPieceColour::Black, ChessPieceKind::Knight);
        board.set(3, 3, ChessPieceColour::White, ChessPieceKind::Knight);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 1,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 2,
                    y: 1,
                    takes: false
                }
            ])),
            board.get_moves(0, 0)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 0,
                    y: 0,
                    takes: true
                },
                Move {
                    x: 3,
                    y: 3,
                    takes: true
                },
                Move {
                    x: 2,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false
                },
            ])),
            board.get_moves(1, 2)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 1,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 3,
                    takes: false
                },
            ])),
            board.get_moves(0, 4)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 1,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 2,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 4,
                    takes: false
                },
            ])),
            board.get_moves(3, 3)
        );
    }

    #[test]
    fn rook_moves() {
        let mut board = Board::new();
        board.set(3, 2, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(7, 2, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(5, 2, ChessPieceColour::Black, ChessPieceKind::Rook);
        board.set(5, 5, ChessPieceColour::Black, ChessPieceKind::Rook);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 4,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 7,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 0,
                    y: 2,
                    takes: false
                },
            ])),
            board.get_moves(3, 2)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 6,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 7,
                    takes: false
                },
            ])),
            board.get_moves(7, 2)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 7,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 4,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 6,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 4,
                    takes: false
                },
            ])),
            board.get_moves(5, 2)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 7,
                    takes: false
                },
                Move {
                    x: 6,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 0,
                    y: 5,
                    takes: false
                },
            ])),
            board.get_moves(5, 5)
        );
    }

    #[test]
    fn bishop_moves() {
        let mut board = Board::new();
        board.set(6, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(2, 5, ChessPieceColour::Black, ChessPieceKind::Pawn);

        board.set(7, 0, ChessPieceColour::White, ChessPieceKind::Bishop);
        board.set(5, 3, ChessPieceColour::White, ChessPieceKind::Bishop);
        board.set(4, 3, ChessPieceColour::Black, ChessPieceKind::Bishop);
        board.set(7, 5, ChessPieceColour::Black, ChessPieceKind::Bishop);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 4,
                    y: 3,
                    takes: true
                },
                Move {
                    x: 5,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 6,
                    y: 1,
                    takes: false
                },
            ])),
            board.get_moves(7, 0)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 7,
                    y: 5,
                    takes: true
                },
                Move {
                    x: 6,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 6,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 7,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 0,
                    takes: false
                },
            ])),
            board.get_moves(5, 3)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 7,
                    y: 0,
                    takes: true
                },
                Move {
                    x: 6,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 5,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 6,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 7,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 0,
                    takes: false
                },
            ])),
            board.get_moves(4, 3)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 3,
                    takes: true
                },
                Move {
                    x: 6,
                    y: 4,
                    takes: false
                }
            ])),
            board.get_moves(7, 5)
        );
    }

    #[test]
    fn queen_moves() {
        let mut board = Board::new();
        board.set(5, 2, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(1, 1, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(5, 5, ChessPieceColour::Black, ChessPieceKind::Pawn);

        board.set(2, 2, ChessPieceColour::Black, ChessPieceKind::Queen);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 2,
                    takes: true
                },
                Move {
                    x: 4,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 0,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 0,
                    y: 2,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 0,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 4,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 5,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 6,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 7,
                    takes: false
                },
                Move {
                    x: 3,
                    y: 3,
                    takes: false
                },
                Move {
                    x: 4,
                    y: 4,
                    takes: false
                },
            ])),
            board.get_moves(2, 2)
        );
    }

    #[test]
    fn king_basic_moves() {
        let mut board = Board::new();
        board.set(3, 3, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(2, 3, ChessPieceColour::Black, ChessPieceKind::Pawn);

        board.set(2, 2, ChessPieceColour::White, ChessPieceKind::King);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 2,
                    y: 3,
                    takes: true
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 2,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 1,
                    takes: false
                },
                Move {
                    x: 1,
                    y: 3,
                    takes: false
                },
            ])),
            board.get_moves(2, 2)
        );
    }

    #[test]
    fn pawn_double_moves() {
        let mut board = Board::new();
        board.set(4, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(4, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);

        board.board[7 - 4][3] = Some(ChessPiece {
            x: 3,
            y: 4,
            colour: ChessPieceColour::White,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 3][6] = Some(ChessPiece {
            x: 6,
            y: 3,
            colour: ChessPieceColour::White,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 4][2] = Some(ChessPiece {
            x: 2,
            y: 4,
            colour: ChessPieceColour::Black,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        board.board[7 - 3][5] = Some(ChessPiece {
            x: 5,
            y: 3,
            colour: ChessPieceColour::Black,
            kind: ChessPieceKind::Pawn,
            has_moved: true,
        });
        // board.set(3, 4, ChessPieceColour::White, ChessPieceKind::Pawn);
        // board.set(6, 3, ChessPieceColour::White, ChessPieceKind::Pawn);
        // board.set(2, 4, ChessPieceColour::Black, ChessPieceKind::Pawn);
        // board.set(5, 3, ChessPieceColour::Black, ChessPieceKind::Pawn);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 4,
                    y: 2,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 3,
                    takes: false,
                },
            ])),
            board.get_moves(4, 1)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 4,
                    y: 5,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 4,
                    takes: false,
                },
            ])),
            board.get_moves(4, 6)
        );

        board.last_move = Some(MoveFromTo::new(2, 6, 2, 4));

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 2,
                    y: 5,
                    takes: false,
                },
                Move {
                    x: 3,
                    y: 5,
                    takes: false,
                },
            ])),
            board.get_moves(3, 4)
        );
        assert_eq!(
            Some(HashSet::from([Move {
                x: 6,
                y: 4,
                takes: false,
            }])),
            board.get_moves(6, 3)
        );
        assert_eq!(
            Some(HashSet::from([Move {
                x: 2,
                y: 3,
                takes: false,
            }])),
            board.get_moves(2, 4)
        );

        board.last_move = Some(MoveFromTo::new(6, 1, 6, 3));

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 5,
                    y: 2,
                    takes: false,
                },
                Move {
                    x: 6,
                    y: 2,
                    takes: false,
                },
            ])),
            board.get_moves(5, 3)
        );
    }

    #[test]
    fn king_castling() {
        let mut board = Board::new();
        board.set(0, 0, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(7, 0, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(0, 7, ChessPieceColour::Black, ChessPieceKind::Rook);
        board.set(7, 7, ChessPieceColour::Black, ChessPieceKind::Rook);

        board.set(4, 0, ChessPieceColour::White, ChessPieceKind::King);
        board.set(4, 7, ChessPieceColour::Black, ChessPieceKind::King);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 0,
                    takes: false,
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 0,
                    takes: false,
                },
                Move {
                    x: 2,
                    y: 0,
                    takes: false,
                },
                Move {
                    x: 6,
                    y: 0,
                    takes: false,
                },
            ])),
            board.get_moves(4, 0)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 7,
                    takes: false,
                },
                Move {
                    x: 3,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 7,
                    takes: false,
                },
                Move {
                    x: 2,
                    y: 7,
                    takes: false,
                },
                Move {
                    x: 6,
                    y: 7,
                    takes: false,
                },
            ])),
            board.get_moves(4, 7)
        );
    }

    #[test]
    fn king_castling_obstructed() {
        let mut board = Board::new();
        board.set(0, 0, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(7, 0, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(1, 0, ChessPieceColour::White, ChessPieceKind::Knight);
        board.set(6, 0, ChessPieceColour::White, ChessPieceKind::Knight);
        board.set(0, 7, ChessPieceColour::Black, ChessPieceKind::Rook);
        board.set(7, 7, ChessPieceColour::Black, ChessPieceKind::Rook);
        board.set(1, 7, ChessPieceColour::Black, ChessPieceKind::Knight);
        board.set(6, 7, ChessPieceColour::Black, ChessPieceKind::Knight);

        board.set(4, 0, ChessPieceColour::White, ChessPieceKind::King);
        board.set(4, 7, ChessPieceColour::Black, ChessPieceKind::King);

        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 0,
                    takes: false,
                },
                Move {
                    x: 3,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 1,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 0,
                    takes: false,
                },
            ])),
            board.get_moves(4, 0)
        );
        assert_eq!(
            Some(HashSet::from([
                Move {
                    x: 3,
                    y: 7,
                    takes: false,
                },
                Move {
                    x: 3,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 4,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 6,
                    takes: false,
                },
                Move {
                    x: 5,
                    y: 7,
                    takes: false,
                },
            ])),
            board.get_moves(4, 7)
        );
    }
}
