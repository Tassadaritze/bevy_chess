use std::collections::HashSet;

use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::Board;
use crate::tilemap::pieces::PieceTilemap;
use crate::utils::cursor;

pub mod board;
pub mod pieces;

pub struct BoardClickEvent(pub Option<HashSet<Move>>);

pub fn mouse_click(
    mouse_input: Res<Input<MouseButton>>,
    cursor_pos: Res<cursor::CursorPos>,
    board: Res<Board>,
    tilemap_q: Query<
        (&TilemapSize, &TilemapGridSize, &TilemapType, &Transform),
        With<PieceTilemap>,
    >,
    mut click_ev: EventWriter<BoardClickEvent>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let (map_size, grid_size, map_type, map_transform) = tilemap_q.single();
        let cursor_pos: Vec3 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 1.0));
            let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
            cursor_in_map_pos.xy()
        };

        if let Some(tile_pos) =
            TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
        {
            let moves = board.get_moves(tile_pos.x, tile_pos.y);
            click_ev.send(BoardClickEvent(moves));
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