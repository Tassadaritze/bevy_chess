use std::collections::HashSet;

use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::{Board, BoardHistory};
use crate::game::pieces::{ChessPieceColour, ChessPieceKind};
use crate::tilemap::board::BoardTilemap;
use crate::tilemap::move_indicators::{MoveIndicator, SelectedTile};
use crate::utils::cursor::CursorPos;

pub mod board;
pub mod pieces;

#[derive(Resource, Default)]
pub struct IsBlackTurn(pub bool);

#[derive(Default)]
pub struct BoardClickEvent {
    pub tile: TilePos,
    pub moves: Option<HashSet<Move>>,
}

#[derive(Default)]
pub struct CheckEvent(pub Option<TilePos>);

#[allow(clippy::too_many_arguments)]
pub fn mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    cursor_pos: Res<CursorPos>,
    mut is_black_turn: ResMut<IsBlackTurn>,
    mut board: ResMut<Board>,
    mut history: ResMut<BoardHistory>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &Transform,
            &TileStorage,
        ),
        With<BoardTilemap>,
    >,
    tiles_w_indicators_q: Query<(), With<MoveIndicator>>,
    tile_selected_q: Query<&TilePos, With<SelectedTile>>,
    mut click_ev: EventWriter<BoardClickEvent>,
    mut check_ev: EventWriter<CheckEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, map_type, map_transform, tile_storage) = tilemap_q.single();
        let cursor_pos: Vec3 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                if tiles_w_indicators_q.get(tile_entity).is_err() {
                    // deselect selected tile if clicked again
                    if let Ok(selected_tile) = tile_selected_q.get_single() {
                        if selected_tile == &tile_pos {
                            click_ev.send(BoardClickEvent::default());
                            return;
                        }
                    }
                    if let Some(piece) = board.get(tile_pos.x, tile_pos.y) {
                        assert!(piece.x == tile_pos.x && piece.y == tile_pos.y);

                        // deselect/do nothing if it's not that side's turn
                        match is_black_turn.0 {
                            true => {
                                if let ChessPieceColour::White = piece.colour {
                                    click_ev.send(BoardClickEvent::default());
                                    return;
                                }
                            }
                            false => {
                                if let ChessPieceColour::Black = piece.colour {
                                    click_ev.send(BoardClickEvent::default());
                                    return;
                                }
                            }
                        }

                        let mut moves = board.get_moves(piece.x, piece.y);

                        if let Some(check) = board.check {
                            if piece.colour == check && piece.kind != ChessPieceKind::King {
                                if let Some(moves) = &mut moves {
                                    if !moves.is_empty() {
                                        let attackers = board.get_king_attackers(check);
                                        // if the king managed to get in check, we're assuming he exists
                                        let king =
                                            board.find_piece(ChessPieceKind::King, check).unwrap();
                                        if attackers.len() > 1 {
                                            moves.clear();
                                        } else {
                                            for attacker in attackers {
                                                if let ChessPieceKind::Rook
                                                | ChessPieceKind::Bishop
                                                | ChessPieceKind::Queen = attacker.kind
                                                {
                                                    let threats = board
                                                        .get_threatened_tiles(
                                                            attacker.x, attacker.y,
                                                        )
                                                        .unwrap()
                                                        .between(
                                                            BoardPos {
                                                                x: king.x,
                                                                y: king.y,
                                                            },
                                                            BoardPos {
                                                                x: attacker.x,
                                                                y: attacker.y,
                                                            },
                                                        );
                                                    moves.retain(|_move| {
                                                        threats.iter().any(|threat| _move == threat)
                                                            || _move
                                                                == &BoardPos {
                                                                    x: attacker.x,
                                                                    y: attacker.y,
                                                                }
                                                    });
                                                } else {
                                                    moves.retain(|_move| {
                                                        _move
                                                            == &BoardPos {
                                                                x: attacker.x,
                                                                y: attacker.y,
                                                            }
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // is this piece pinned?
                        if let Some(moves) = &mut moves {
                            if !moves.is_empty() {
                                if let Some(king) =
                                    board.find_piece(ChessPieceKind::King, piece.colour)
                                {
                                    for potential_attacker in
                                        board.iter().flatten().filter(|piece| {
                                            if piece.colour != king.colour {
                                                if let ChessPieceKind::Rook
                                                | ChessPieceKind::Bishop
                                                | ChessPieceKind::Queen = piece.kind
                                                {
                                                    return true;
                                                }
                                            }
                                            false
                                        })
                                    {
                                        if let Some(threats) = board.get_threatened_tiles(
                                            potential_attacker.x,
                                            potential_attacker.y,
                                        ) {
                                            if threats
                                                .between(
                                                    BoardPos {
                                                        x: king.x,
                                                        y: king.y,
                                                    },
                                                    BoardPos {
                                                        x: potential_attacker.x,
                                                        y: potential_attacker.y,
                                                    },
                                                )
                                                .contains(&BoardPos {
                                                    x: piece.x,
                                                    y: piece.y,
                                                })
                                            {
                                                moves.clear();
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        click_ev.send(BoardClickEvent {
                            tile: tile_pos,
                            moves,
                        });
                    } else {
                        click_ev.send(BoardClickEvent::default());
                    }
                } else if let Ok(selected_tile) = tile_selected_q.get_single() {
                    history.0.push(board.clone());
                    board._move(selected_tile.x, selected_tile.y, tile_pos.x, tile_pos.y);
                    is_black_turn.0 = !is_black_turn.0;
                    click_ev.send(BoardClickEvent::default());

                    // did that move cause a check?
                    let mut check: Option<ChessPieceColour> = None;
                    if let Some(king) = board.find_piece(
                        ChessPieceKind::King,
                        !board.get(tile_pos.x, tile_pos.y).unwrap().colour,
                    ) {
                        if !board.get_king_attackers(king.colour).is_empty() {
                            check_ev.send(CheckEvent(Some(TilePos::new(king.x, king.y))));
                            check = Some(king.colour);
                        } else {
                            check_ev.send(CheckEvent::default());
                        }
                    }
                    board.check = check;
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Move {
    x: u32,
    y: u32,
    pub takes: bool,
}

impl From<&Move> for TilePos {
    fn from(_move: &Move) -> Self {
        Self {
            x: _move.x,
            y: _move.y,
        }
    }
}

impl PartialEq<BoardPos> for Move {
    fn eq(&self, other: &BoardPos) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct BoardPos {
    x: u32,
    y: u32,
}

impl PartialEq<Move> for BoardPos {
    fn eq(&self, other: &Move) -> bool {
        self.x == other.x && self.y == other.y
    }
}

trait Between {
    fn between(&self, pos1: BoardPos, pos2: BoardPos) -> Self;
}

impl Between for HashSet<BoardPos> {
    // returns the set of tiles that are in a straight line between pos1 and pos2
    fn between(&self, pos1: BoardPos, pos2: BoardPos) -> Self {
        let mut between = HashSet::new();

        let x_diff = pos1.x as i32 - pos2.x as i32;
        let y_diff = pos1.y as i32 - pos2.y as i32;
        if x_diff.abs() != y_diff.abs() && x_diff != 0 && y_diff != 0 {
            return between;
        }
        let x_offset = x_diff.clamp(-1, 1);
        let y_offset = y_diff.clamp(-1, 1);
        let mut next_pos = BoardPos {
            x: (pos1.x as i32 - x_offset) as u32,
            y: (pos1.y as i32 - y_offset) as u32,
        };
        while self.contains(&next_pos) {
            between.insert(next_pos);

            next_pos = BoardPos {
                x: (next_pos.x as i32 - x_offset) as u32,
                y: (next_pos.y as i32 - y_offset) as u32,
            };
        }

        between
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct MoveFromTo {
    from: BoardPos,
    to: BoardPos,
}

impl MoveFromTo {
    pub fn new(from_x: u32, from_y: u32, to_x: u32, to_y: u32) -> Self {
        Self {
            from: BoardPos {
                x: from_x,
                y: from_y,
            },
            to: BoardPos { x: to_x, y: to_y },
        }
    }
}
