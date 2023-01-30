use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::Board;
use crate::game::pieces::{ChessPieceColour, ChessPieceKind};

const WHITE_KING_TEXTURE: u8 = 0;
const WHITE_QUEEN_TEXTURE: u8 = 1;
const WHITE_BISHOP_TEXTURE: u8 = 2;
const WHITE_KNIGHT_TEXTURE: u8 = 3;
const WHITE_ROOK_TEXTURE: u8 = 4;
const WHITE_PAWN_TEXTURE: u8 = 5;
const BLACK_KING_TEXTURE: u8 = 6;
const BLACK_QUEEN_TEXTURE: u8 = 7;
const BLACK_BISHOP_TEXTURE: u8 = 8;
const BLACK_KNIGHT_TEXTURE: u8 = 9;
const BLACK_ROOK_TEXTURE: u8 = 10;
const BLACK_PAWN_TEXTURE: u8 = 11;

#[derive(Component)]
pub struct PieceTilemap;

pub fn create_piece_tilemap(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    board: Res<Board>,
) {
    let texture_handle: Handle<Image> = asset_server.load("chess_pieces_96px.png");

    let map_size = TilemapSize { x: 8, y: 8 };
    let tilemap_entity = commands.spawn(PieceTilemap).id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let board_piece = match board.get(x, y) {
                Some(piece) => piece,
                None => continue,
            };
            let texture = match board_piece.kind {
                ChessPieceKind::Pawn => match board_piece.colour {
                    ChessPieceColour::White => WHITE_PAWN_TEXTURE,
                    ChessPieceColour::Black => BLACK_PAWN_TEXTURE,
                },
                ChessPieceKind::Knight => match board_piece.colour {
                    ChessPieceColour::White => WHITE_KNIGHT_TEXTURE,
                    ChessPieceColour::Black => BLACK_KNIGHT_TEXTURE,
                },
                ChessPieceKind::Rook => match board_piece.colour {
                    ChessPieceColour::White => WHITE_ROOK_TEXTURE,
                    ChessPieceColour::Black => BLACK_ROOK_TEXTURE,
                },
                ChessPieceKind::Bishop => match board_piece.colour {
                    ChessPieceColour::White => WHITE_BISHOP_TEXTURE,
                    ChessPieceColour::Black => BLACK_BISHOP_TEXTURE,
                },
                ChessPieceKind::Queen => match board_piece.colour {
                    ChessPieceColour::White => WHITE_QUEEN_TEXTURE,
                    ChessPieceColour::Black => BLACK_QUEEN_TEXTURE,
                },
                ChessPieceKind::King => match board_piece.colour {
                    ChessPieceColour::White => WHITE_KING_TEXTURE,
                    ChessPieceColour::Black => BLACK_KING_TEXTURE,
                },
            };
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture as u32),
                    ..default()
                },))
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 96.0, y: 96.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..default()
    });
}
