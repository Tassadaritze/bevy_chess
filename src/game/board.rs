use bevy::prelude::*;
use std::collections::HashSet;

use crate::game::pieces::{ChessPiece, ChessPieceColour, ChessPieceKind};
use crate::game::Move;

#[derive(Resource, Debug)]
pub struct Board(Vec<Vec<Option<ChessPiece>>>);

impl Default for Board {
    fn default() -> Self {
        let mut board = Self::new();

        board.set(0, 7, ChessPieceColour::Black, ChessPieceKind::Rook);
        board.set(1, 7, ChessPieceColour::Black, ChessPieceKind::Knight);
        board.set(2, 7, ChessPieceColour::Black, ChessPieceKind::Bishop);
        board.set(3, 7, ChessPieceColour::Black, ChessPieceKind::Queen);
        board.set(4, 7, ChessPieceColour::Black, ChessPieceKind::King);
        board.set(5, 7, ChessPieceColour::Black, ChessPieceKind::Bishop);
        board.set(6, 7, ChessPieceColour::Black, ChessPieceKind::Knight);
        board.set(7, 7, ChessPieceColour::Black, ChessPieceKind::Rook);

        board.set(0, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(1, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(2, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(3, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(4, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(5, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(6, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);
        board.set(7, 6, ChessPieceColour::Black, ChessPieceKind::Pawn);

        board.set(0, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(1, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(2, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(3, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(4, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(5, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(6, 1, ChessPieceColour::White, ChessPieceKind::Pawn);
        board.set(7, 1, ChessPieceColour::White, ChessPieceKind::Pawn);

        board.set(0, 0, ChessPieceColour::White, ChessPieceKind::Rook);
        board.set(1, 0, ChessPieceColour::White, ChessPieceKind::Knight);
        board.set(2, 0, ChessPieceColour::White, ChessPieceKind::Bishop);
        board.set(3, 0, ChessPieceColour::White, ChessPieceKind::Queen);
        board.set(4, 0, ChessPieceColour::White, ChessPieceKind::King);
        board.set(5, 0, ChessPieceColour::White, ChessPieceKind::Bishop);
        board.set(6, 0, ChessPieceColour::White, ChessPieceKind::Knight);
        board.set(7, 0, ChessPieceColour::White, ChessPieceKind::Rook);

        // TODO: return this to actual starting game state
        board.set(3, 4, ChessPieceColour::Black, ChessPieceKind::Queen);
        board.set(4, 4, ChessPieceColour::White, ChessPieceKind::Queen);

        board
    }
}

impl Board {
    pub fn new() -> Self {
        Self(vec![
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
            vec![None, None, None, None, None, None, None, None],
        ])
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&ChessPiece> {
        self.0[7 - y as usize]
            .get(x as usize)
            .and_then(|piece| piece.as_ref())
    }

    pub fn set(&mut self, x: u32, y: u32, colour: ChessPieceColour, kind: ChessPieceKind) {
        self.0[7 - y as usize][x as usize] = Some(ChessPiece::new(colour, kind, x, y));
    }

    fn delete(&mut self, x: u32, y: u32) {
        self.0[7 - y as usize][x as usize] = None;
    }

    fn get_pawn_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        let mut moves = HashSet::new();

        let y_offset: i32 = match colour {
            ChessPieceColour::White => 1,
            ChessPieceColour::Black => -1,
        };
        // a situation where we check if a pawn can move to y < 0 (or y > 7) should not happen
        let potential_y = (y as i32 + y_offset) as u32;
        for potential_x in x.saturating_sub(1)..=x + 1 {
            let potential_tile = self.get(potential_x, potential_y);
            if potential_x == x && potential_tile.is_none() {
                moves.insert(Move {
                    x: potential_x,
                    y: potential_y,
                    takes: false,
                });
            } else if potential_x != x {
                if let Some(val) = potential_tile {
                    if &val.colour != colour {
                        moves.insert(Move {
                            x: potential_x,
                            y: potential_y,
                            takes: true,
                        });
                    }
                }
            }
        }

        moves
    }

    fn get_knight_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        const KNIGHT_MOVE_OFFSETS: [(i32, i32); 8] = [
            (1, 2),
            (2, 1),
            (-1, 2),
            (-2, 1),
            (1, -2),
            (2, -1),
            (-1, -2),
            (-2, -1),
        ];

        let mut moves = HashSet::new();

        for (x_offset, y_offset) in KNIGHT_MOVE_OFFSETS {
            let potential_x = x as i32 + x_offset;
            let potential_y = y as i32 + y_offset;
            if (0..8).contains(&potential_x) && (0..8).contains(&potential_y) {
                let potential_x = potential_x as u32;
                let potential_y = potential_y as u32;
                let potential_tile = self.get(potential_x, potential_y);
                let takes: bool;
                if let Some(piece) = potential_tile {
                    if &piece.colour != colour {
                        takes = true;
                    } else {
                        continue;
                    }
                } else {
                    takes = false;
                }
                moves.insert(Move {
                    x: potential_x,
                    y: potential_y,
                    takes,
                });
            }
        }

        moves
    }

    fn get_rook_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        let mut moves = HashSet::new();

        // TODO: maybe do this differently
        for potential_x in (0..x).rev() {
            if let Some(_move) = self.can_do_move(potential_x, y, colour) {
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
            } else {
                break;
            }
        }
        for potential_x in x + 1..8 {
            if let Some(_move) = self.can_do_move(potential_x, y, colour) {
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
            } else {
                break;
            }
        }

        for potential_y in (0..y).rev() {
            if let Some(_move) = self.can_do_move(x, potential_y, colour) {
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
            } else {
                break;
            }
        }
        for potential_y in y + 1..8 {
            if let Some(_move) = self.can_do_move(x, potential_y, colour) {
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
            } else {
                break;
            }
        }

        moves
    }

    fn get_bishop_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        const BISHOP_MOVE_OFFSETS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        let mut moves = HashSet::new();

        for (x_offset, y_offset) in BISHOP_MOVE_OFFSETS {
            let mut potential_x = x as i32 + x_offset;
            let mut potential_y = y as i32 + y_offset;
            while let Some(_move) = self.can_do_move(potential_x as u32, potential_y as u32, colour)
            {
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
                potential_x += x_offset;
                potential_y += y_offset;
            }
        }

        moves
    }

    fn get_queen_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        &self.get_rook_moves(x, y, colour) | &self.get_bishop_moves(x, y, colour)
    }

    fn get_king_moves(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<Move> {
        let mut moves = HashSet::new();

        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == y_offset && x_offset == 0 {
                    continue;
                }
                let potential_x = x as i32 + x_offset;
                let potential_y = y as i32 + y_offset;
                if let Some(_move) =
                    self.can_do_move(potential_x as u32, potential_y as u32, colour)
                {
                    moves.insert(_move);
                }
            }
        }

        moves
    }

    pub fn get_moves(&self, x: u32, y: u32) -> Option<HashSet<Move>> {
        self.get(x, y).map(|piece| match piece.kind {
            ChessPieceKind::Pawn => self.get_pawn_moves(x, y, &piece.colour),
            ChessPieceKind::Knight => self.get_knight_moves(x, y, &piece.colour),
            ChessPieceKind::Rook => self.get_rook_moves(x, y, &piece.colour),
            ChessPieceKind::Bishop => self.get_bishop_moves(x, y, &piece.colour),
            ChessPieceKind::Queen => self.get_queen_moves(x, y, &piece.colour),
            ChessPieceKind::King => self.get_king_moves(x, y, &piece.colour),
        })
    }

    // if moving a piece with given colour to x,y is valid, returns that move as a Some(Move)
    // else returns None
    fn can_do_move(&self, x: u32, y: u32, colour: &ChessPieceColour) -> Option<Move> {
        if !(0..8).contains(&x) || !(0..8).contains(&y) {
            return None;
        }
        match self.get(x, y) {
            Some(other_piece) => {
                if &other_piece.colour != colour {
                    Some(Move { x, y, takes: true })
                } else {
                    None
                }
            }
            None => Some(Move { x, y, takes: false }),
        }
    }
}
