use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::{BoardClickEvent, Move};
use crate::tilemap::board;

const MOVE_INDICATOR_Z: f32 = 2.0;

#[derive(Resource)]
pub struct MoveIndicatorHandle(Handle<Image>);

#[derive(Resource)]
pub struct TakeIndicatorHandle(Handle<Image>);

impl FromWorld for MoveIndicatorHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("move_indicator.png"))
    }
}

impl FromWorld for TakeIndicatorHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("take_indicator.png"))
    }
}

#[derive(Component)]
pub struct SelectedTile;

#[derive(Component)]
pub struct MoveIndicator(Entity);

#[allow(clippy::too_many_arguments)]
pub fn spawn_move_indicators(
    mut commands: Commands,
    move_indicator_handle: Res<MoveIndicatorHandle>,
    take_indicator_handle: Res<TakeIndicatorHandle>,
    mut click_ev: EventReader<BoardClickEvent>,
    tilemap_q: Query<
        (&TilemapGridSize, &TilemapType, &TileStorage, &Transform),
        With<board::BoardTilemap>,
    >,
    tiles_w_indicator_q: Query<Entity, With<MoveIndicator>>,
    tile_selected_q: Query<Entity, With<SelectedTile>>,
    indicator_q: Query<&MoveIndicator>,
) {
    for ev in click_ev.iter() {
        for tile_entity in tiles_w_indicator_q.iter() {
            if let Ok(move_indicator) = indicator_q.get(tile_entity) {
                commands.entity(move_indicator.0).despawn();
                commands.entity(tile_entity).remove::<MoveIndicator>();
            }
        }
        for tile_entity in tile_selected_q.iter() {
            commands.entity(tile_entity).remove::<SelectedTile>();
        }

        let (grid_size, map_type, tile_storage, map_transform) = tilemap_q.single();

        if let Some(tile_entity) = tile_storage.get(&ev.tile) {
            commands.entity(tile_entity).insert(SelectedTile);
        }

        if let Some(moves) = &ev.moves {
            for _move in moves {
                if let Some(tile_entity) = tile_storage.get(&_move.into()) {
                    let tile_center = <&Move as Into<TilePos>>::into(_move)
                        .center_in_world(grid_size, map_type)
                        .extend(MOVE_INDICATOR_Z);
                    let transform = *map_transform * Transform::from_translation(tile_center);

                    let texture = if _move.takes {
                        take_indicator_handle.0.clone()
                    } else {
                        move_indicator_handle.0.clone()
                    };

                    let move_indicator = commands
                        .spawn(SpriteBundle {
                            texture,
                            transform,
                            ..default()
                        })
                        .id();

                    commands
                        .entity(tile_entity)
                        .insert(MoveIndicator(move_indicator));
                }
            }
        } else {
            continue;
        }
    }
}
