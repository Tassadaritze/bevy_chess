use bevy::prelude::*;
use std::collections::HashSet;

use crate::game::pieces::{ChessPiece, ChessPieceColour, ChessPieceKind};
use crate::game::{Move, MoveFromTo};

#[derive(Resource, Debug)]
pub struct Board {
    pub(crate) board: Vec<Vec<Option<ChessPiece>>>,
    pub(crate) last_move: Option<MoveFromTo>,
}

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
        Self {
            board: vec![
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
                vec![None, None, None, None, None, None, None, None],
            ],
            last_move: None,
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&ChessPiece> {
        self.board[7 - y as usize]
            .get(x as usize)
            .and_then(|piece| piece.as_ref())
    }

    pub fn set(&mut self, x: u32, y: u32, colour: ChessPieceColour, kind: ChessPieceKind) {
        self.board[7 - y as usize][x as usize] = Some(ChessPiece::new(colour, kind));
    }

    fn delete(&mut self, x: u32, y: u32) {
        self.board[7 - y as usize][x as usize] = None;
    }

    fn _move(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        if let Some(piece) = self.get(from_x, from_y) {
            let mut piece = *piece;
            if !piece.has_moved {
                piece.has_moved = true;
            }
            self.board[7 - to_y as usize][to_x as usize] = Some(piece);
            self.delete(from_x, from_y);
            self.last_move = Some(MoveFromTo::new(from_x, from_y, to_x, to_y));
        } else {
            error!(
                "tried to move an empty tile from {},{} to {},{}",
                from_x, from_y, to_x, to_y
            );
        }
    }

    fn get_pawn_moves(
        &self,
        x: u32,
        y: u32,
        colour: &ChessPieceColour,
        has_moved: bool,
    ) -> HashSet<Move> {
        let mut moves = HashSet::new();

        let y_offset: i32 = match colour {
            ChessPieceColour::White => 1,
            ChessPieceColour::Black => -1,
        };
        // a situation where we check if a pawn can move to y < 0 (or y > 7) should not happen
        let potential_y = y as i32 + y_offset;

        // basic move + double move
        if let Some(_move) = self.can_do_move(x, potential_y as u32, colour) {
            if !_move.takes {
                moves.insert(_move);
                if !has_moved {
                    if let Some(_move) =
                        self.can_do_move(x, (potential_y + y_offset) as u32, colour)
                    {
                        if !_move.takes {
                            moves.insert(_move);
                        }
                    }
                }
            }
        }

        // taking + en passant
        for x_offset in [-1, 1] {
            let potential_x = x as i32 + x_offset;
            if let Some(_move) = self.can_do_move(potential_x as u32, potential_y as u32, colour) {
                if _move.takes {
                    moves.insert(_move);
                } else if let Some(last_move) = &self.last_move {
                    if last_move
                        == &MoveFromTo::new(
                            potential_x as u32,
                            (potential_y + y_offset) as u32,
                            potential_x as u32,
                            y,
                        )
                    {
                        if let Some(piece) = self.get(potential_x as u32, y) {
                            if let ChessPieceKind::Pawn = piece.kind {
                                // if last turn a pawn could make a move that allows for en passant,
                                // then there shouldn't be a piece that belongs to the opponent
                                // in the tile that we have to move to to perform en passant
                                if let Some(_move) =
                                    self.can_do_move(potential_x as u32, potential_y as u32, colour)
                                {
                                    moves.insert(_move);
                                }
                            }
                        }
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
        const ROOK_MOVE_OFFSETS: [i32; 2] = [-1, 1];

        let mut moves = HashSet::new();

        for x_offset in ROOK_MOVE_OFFSETS {
            let mut potential_x = x as i32 + x_offset;
            while let Some(_move) = self.can_do_move(potential_x as u32, y, colour) {
                potential_x += x_offset;
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
            }
        }

        for y_offset in ROOK_MOVE_OFFSETS {
            let mut potential_y = y as i32 + y_offset;
            while let Some(_move) = self.can_do_move(x, potential_y as u32, colour) {
                potential_y += y_offset;
                let takes = _move.takes;
                moves.insert(_move);
                if takes {
                    break;
                }
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

    fn get_king_moves(
        &self,
        x: u32,
        y: u32,
        colour: &ChessPieceColour,
        has_moved: bool,
    ) -> HashSet<Move> {
        let mut moves = HashSet::new();

        // basic moves
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

        // castling
        if !has_moved {
            'x_sides: for x_offset in [-1, 1] {
                let mut potential_x = x as i32 + x_offset;
                while let Some(_move) = self.can_do_move(potential_x as u32, y, colour) {
                    if _move.takes {
                        continue 'x_sides;
                    }
                    potential_x += x_offset;
                }
                if [0, 7].contains(&potential_x) {
                    if let Some(piece) = self.get(potential_x as u32, y) {
                        if let ChessPieceKind::Rook = piece.kind {
                            // if we got this far without breaking the loop, then the chess piece
                            // at x:0 or x:7 has to be the same colour
                            // ...plus the king is in check anyway if the rook in that position
                            // is the wrong colour
                            if !piece.has_moved {
                                moves.insert(Move {
                                    x: (x as i32 + x_offset * 2) as u32,
                                    y,
                                    takes: false,
                                });
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    pub fn get_moves(&self, x: u32, y: u32) -> Option<HashSet<Move>> {
        self.get(x, y).map(|piece| match piece.kind {
            ChessPieceKind::Pawn => self.get_pawn_moves(x, y, &piece.colour, piece.has_moved),
            ChessPieceKind::Knight => self.get_knight_moves(x, y, &piece.colour),
            ChessPieceKind::Rook => self.get_rook_moves(x, y, &piece.colour),
            ChessPieceKind::Bishop => self.get_bishop_moves(x, y, &piece.colour),
            ChessPieceKind::Queen => self.get_queen_moves(x, y, &piece.colour),
            ChessPieceKind::King => self.get_king_moves(x, y, &piece.colour, piece.has_moved),
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

#[derive(Resource)]
struct BoardHistory(Vec<Board>);
