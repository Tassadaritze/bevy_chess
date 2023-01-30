extern crate core;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::board::Board;
use crate::game::{mouse_click, BoardClickEvent};
use crate::tilemap::board::create_board_tilemap;
use crate::tilemap::hover::{show_hover_ring, spawn_hover_ring};
use crate::tilemap::move_indicators::{
    spawn_move_indicators, MoveIndicatorHandle, TakeIndicatorHandle,
};
use crate::tilemap::pieces::create_piece_tilemap;
use crate::tilemap::ranks_and_files::create_labels;
use crate::utils::cursor::{update_cursor_pos, CursorPos};

mod game;
mod tilemap;
mod utils;

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: String::from("Chess"),
                        ..default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_resource::<CursorPos>()
        .init_resource::<Board>()
        .init_resource::<MoveIndicatorHandle>()
        .init_resource::<TakeIndicatorHandle>()
        .add_event::<BoardClickEvent>()
        .add_plugin(TilemapPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(create_board_tilemap)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_piece_tilemap)
        .add_startup_system_to_stage(StartupStage::PostStartup, create_labels)
        .add_startup_system_to_stage(StartupStage::PostStartup, spawn_hover_ring)
        .add_system(update_cursor_pos)
        .add_system(spawn_move_indicators)
        .add_system(show_hover_ring.after(update_cursor_pos))
        .add_system(mouse_click.after(update_cursor_pos))
        .run();
}
