use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::{Board, BoardHistory};
use crate::game::{mouse_click, BoardClickEvent, CheckEvent, IsBlackTurn};
use crate::tilemap::board::create_board_tilemap;
use crate::tilemap::checked_tile::{spawn_checked_tile, CheckedTileHandle};
use crate::tilemap::hover::{show_hover_ring, spawn_hover_ring};
use crate::tilemap::move_indicators::{
    spawn_move_indicators, MoveIndicatorHandle, TakeIndicatorHandle,
};
use crate::tilemap::pieces::{draw_piece_tilemap, ChessPieceHandle};
use crate::tilemap::ranks_and_files::create_labels;
use crate::tilemap::turn_indicator::{draw_turn_indicators, spawn_turn_indicators};
use crate::utils::cursor::{update_cursor_pos, CursorPos};
use crate::utils::on_window_resize;

mod game;
mod tilemap;
mod utils;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(-48.0, -48.0, 999.9),
            ..default()
        },
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("Chess"),
                        width: 1600.,
                        height: 900.,
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<CursorPos>()
        .init_resource::<Board>()
        .init_resource::<BoardHistory>()
        .init_resource::<IsBlackTurn>()
        .init_resource::<MoveIndicatorHandle>()
        .init_resource::<TakeIndicatorHandle>()
        .init_resource::<ChessPieceHandle>()
        .init_resource::<CheckedTileHandle>()
        .add_event::<BoardClickEvent>()
        .add_event::<CheckEvent>()
        .add_plugin(TilemapPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(create_board_tilemap)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_labels)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_hover_ring)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_turn_indicators)
        .add_system(draw_piece_tilemap)
        .add_system(draw_turn_indicators)
        .add_system(update_cursor_pos)
        .add_system(spawn_move_indicators)
        .add_system(spawn_checked_tile)
        .add_system(show_hover_ring.after(update_cursor_pos))
        .add_system(mouse_click.after(update_cursor_pos))
        .add_system(on_window_resize)
        .run();
}
