use std::collections::HashSet;

use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::{Board, BoardHistory};
use crate::tilemap::board::BoardTilemap;
use crate::tilemap::move_indicators::{MoveIndicator, SelectedTile};
use crate::utils::cursor;

pub mod board;
pub mod pieces;

#[derive(Default)]
pub struct BoardClickEvent {
    pub tile: TilePos,
    pub moves: Option<HashSet<Move>>,
}

#[allow(clippy::too_many_arguments)]
pub fn mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    cursor_pos: Res<cursor::CursorPos>,
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
                    if let Ok(selected_tile) = tile_selected_q.get_single() {
                        if selected_tile == &tile_pos {
                            click_ev.send(BoardClickEvent::default());
                            return;
                        }
                    }
                    let moves = board.get_moves(tile_pos.x, tile_pos.y);
                    click_ev.send(BoardClickEvent {
                        tile: tile_pos,
                        moves,
                    });
                } else if let Ok(selected_tile) = tile_selected_q.get_single() {
                    history.0.push(board.clone());
                    board._move(selected_tile.x, selected_tile.y, tile_pos.x, tile_pos.y);
                    click_ev.send(BoardClickEvent::default());
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct BoardPos {
    x: u32,
    y: u32,
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
