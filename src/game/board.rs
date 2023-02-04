use std::collections::HashSet;

use bevy::prelude::*;

use crate::game::pieces::{ChessPiece, ChessPieceColour, ChessPieceKind};
use crate::game::{BoardPos, Move, MoveFromTo};

#[derive(Resource, Debug, Clone)]
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
        self.board
            .get(7_usize.checked_sub(y as usize)?)
            .and_then(|row| row.get(x as usize).and_then(|piece| piece.as_ref()))
    }

    pub fn set(&mut self, x: u32, y: u32, colour: ChessPieceColour, kind: ChessPieceKind) {
        self.board[7 - y as usize][x as usize] = Some(ChessPiece::new(x, y, colour, kind));
    }

    fn delete(&mut self, x: u32, y: u32) {
        self.board[7 - y as usize][x as usize] = None;
    }

    pub fn _move(&mut self, from_x: u32, from_y: u32, to_x: u32, to_y: u32) {
        if let Some(piece) = self.get(from_x, from_y) {
            let mut piece = *piece;
            if !piece.has_moved {
                piece.has_moved = true;
            }
            piece.x = to_x;
            piece.y = to_y;
            self.board[7 - to_y as usize][to_x as usize] = Some(piece);
            self.delete(from_x, from_y);

            match piece.kind {
                ChessPieceKind::King => {
                    if from_y == to_y && from_x.abs_diff(to_x) == 2 {
                        let (towards_rook, rook_x): (i32, u32) = match to_x.checked_sub(from_x) {
                            None => (-1, 0),
                            Some(_) => (1, 7),
                        };
                        self._move(rook_x, from_y, (to_x as i32 - towards_rook) as u32, from_y);
                    }
                }
                ChessPieceKind::Pawn => {
                    let y_offset = match piece.colour {
                        ChessPieceColour::White => 1,
                        ChessPieceColour::Black => -1,
                    };
                    if from_x.abs_diff(to_x) == 1
                        && from_y.abs_diff(to_y) == 1
                        && self.last_move
                            == Some(MoveFromTo::new(
                                to_x,
                                (to_y as i32 + y_offset) as u32,
                                to_x,
                                from_y,
                            ))
                    {
                        self.delete(to_x, from_y);
                    }
                }
                _ => (),
            };

            self.last_move = Some(MoveFromTo::new(from_x, from_y, to_x, to_y));
        } else {
            error!(
                "tried to move an empty tile from {},{} to {},{}",
                from_x, from_y, to_x, to_y
            );
        }
    }

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
    const ROOK_MOVE_OFFSETS: [i32; 2] = [-1, 1];
    const BISHOP_MOVE_OFFSETS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

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
        let mut moves = HashSet::new();

        for (x_offset, y_offset) in Self::KNIGHT_MOVE_OFFSETS {
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

        for x_offset in Self::ROOK_MOVE_OFFSETS {
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

        for y_offset in Self::ROOK_MOVE_OFFSETS {
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
        let mut moves = HashSet::new();

        for (x_offset, y_offset) in Self::BISHOP_MOVE_OFFSETS {
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

        let mut threats: HashSet<BoardPos> = HashSet::new();
        for piece in self
            .iter()
            .flatten()
            .filter(|piece| &piece.colour != colour)
        {
            match self.get_threatened_tiles(piece.x, piece.y) {
                Some(new_threats) => threats = &threats | &new_threats,
                None => continue,
            };
        }
        let moves_to_remove: Vec<Move> = moves
            .iter()
            .filter(|&_move| threats.iter().any(|threat| _move == threat))
            .cloned()
            .collect();
        for _move in moves_to_remove {
            moves.remove(&_move);
        }

        // can't jump over threats to castle
        if !has_moved {
            for x_offset in [-1, 1] {
                let one_away = x as i32 + x_offset;
                let two_away = one_away + x_offset;

                let castling_move = moves
                    .iter()
                    .cloned()
                    .find(|_move| _move.x == two_away as u32 && _move.y == y);
                if let Some(_move) = castling_move {
                    if moves
                        .iter()
                        .all(|_move| !(_move.x == one_away as u32 && _move.y == y))
                    {
                        moves.remove(&_move);
                    }
                }
            }
        }

        moves
    }

    fn get_pawn_threats(&self, x: u32, y: u32, colour: &ChessPieceColour) -> HashSet<BoardPos> {
        let mut threats = HashSet::new();

        let y_offset: i32 = match colour {
            ChessPieceColour::White => 1,
            ChessPieceColour::Black => -1,
        };
        let potential_y = (y as i32 + y_offset) as u32;

        if (0..8).contains(&potential_y) {
            for x_offset in [-1, 1] {
                let potential_x = (x as i32 + x_offset) as u32;
                if (0..8).contains(&potential_x) {
                    threats.insert(BoardPos {
                        x: potential_x,
                        y: potential_y,
                    });
                }
            }
        }

        threats
    }

    fn get_knight_threats(&self, x: u32, y: u32) -> HashSet<BoardPos> {
        let mut threats = HashSet::new();

        for (x_offset, y_offset) in Self::KNIGHT_MOVE_OFFSETS {
            let potential_x = (x as i32 + x_offset) as u32;
            let potential_y = (y as i32 + y_offset) as u32;
            if (0..8).contains(&potential_x) && (0..8).contains(&potential_y) {
                threats.insert(BoardPos {
                    x: potential_x,
                    y: potential_y,
                });
            }
        }

        threats
    }

    fn get_rook_threats(&self, x: u32, y: u32) -> HashSet<BoardPos> {
        let mut threats = HashSet::new();

        for x_offset in Self::ROOK_MOVE_OFFSETS {
            let mut potential_x = x as i32 + x_offset;
            if (0..8).contains(&potential_x) {
                while self.get(potential_x as u32, y).is_none() {
                    threats.insert(BoardPos {
                        x: potential_x as u32,
                        y,
                    });
                    potential_x += x_offset;
                    if !(0..8).contains(&potential_x) {
                        break;
                    }
                }
                if self.get(potential_x as u32, y).is_some() {
                    threats.insert(BoardPos {
                        x: potential_x as u32,
                        y,
                    });
                }
            }
        }

        for y_offset in Self::ROOK_MOVE_OFFSETS {
            let mut potential_y = y as i32 + y_offset;
            if (0..8).contains(&potential_y) {
                while self.get(x, potential_y as u32).is_none() {
                    threats.insert(BoardPos {
                        x,
                        y: potential_y as u32,
                    });
                    potential_y += y_offset;
                    if !(0..8).contains(&potential_y) {
                        break;
                    }
                }
                if self.get(x, potential_y as u32).is_some() {
                    threats.insert(BoardPos {
                        x,
                        y: potential_y as u32,
                    });
                }
            }
        }

        threats
    }

    fn get_bishop_threats(&self, x: u32, y: u32) -> HashSet<BoardPos> {
        let mut threats = HashSet::new();

        for (x_offset, y_offset) in Self::BISHOP_MOVE_OFFSETS {
            let mut potential_x = x as i32 + x_offset;
            let mut potential_y = y as i32 + y_offset;
            if (0..8).contains(&potential_x) && (0..8).contains(&potential_y) {
                while self.get(potential_x as u32, potential_y as u32).is_none() {
                    threats.insert(BoardPos {
                        x: potential_x as u32,
                        y: potential_y as u32,
                    });
                    potential_x += x_offset;
                    potential_y += y_offset;
                    if !((0..8).contains(&potential_x) && (0..8).contains(&potential_y)) {
                        break;
                    }
                }
                if self.get(potential_x as u32, potential_y as u32).is_some() {
                    threats.insert(BoardPos {
                        x: potential_x as u32,
                        y: potential_y as u32,
                    });
                }
            }
        }

        threats
    }

    fn get_queen_threats(&self, x: u32, y: u32) -> HashSet<BoardPos> {
        &self.get_rook_threats(x, y) | &self.get_bishop_threats(x, y)
    }

    fn get_king_threats(&self, x: u32, y: u32) -> HashSet<BoardPos> {
        let mut threats = HashSet::new();

        // basic moves
        for y_offset in -1..=1 {
            for x_offset in -1..=1 {
                if x_offset == y_offset && x_offset == 0 {
                    continue;
                }
                let potential_x = (x as i32 + x_offset) as u32;
                let potential_y = (y as i32 + y_offset) as u32;
                threats.insert(BoardPos {
                    x: potential_x,
                    y: potential_y,
                });
            }
        }

        threats
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

    pub fn get_threatened_tiles(&self, x: u32, y: u32) -> Option<HashSet<BoardPos>> {
        self.get(x, y).map(|piece| match piece.kind {
            ChessPieceKind::Pawn => self.get_pawn_threats(x, y, &piece.colour),
            ChessPieceKind::Knight => self.get_knight_threats(x, y),
            ChessPieceKind::Rook => self.get_rook_threats(x, y),
            ChessPieceKind::Bishop => self.get_bishop_threats(x, y),
            ChessPieceKind::Queen => self.get_queen_threats(x, y),
            ChessPieceKind::King => self.get_king_threats(x, y),
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

    pub fn iter(&self) -> Box<dyn Iterator<Item = Option<&ChessPiece>> + '_> {
        let mut iter: Box<dyn Iterator<Item = Option<&ChessPiece>>> =
            Box::new(self.board[0].iter().map(|el| el.as_ref()));
        for row in self.board.iter().skip(1) {
            iter = Box::new(iter.chain(row.iter().map(|el| el.as_ref())));
        }
        iter
    }
}

#[derive(Resource, Default)]
pub struct BoardHistory(pub Vec<Board>);
