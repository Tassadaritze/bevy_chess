use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::CheckEvent;
use crate::tilemap::board;

#[derive(Resource)]
pub struct CheckedTileHandle(Handle<Image>);

impl FromWorld for CheckedTileHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self(asset_server.load("checked_tile.png"))
    }
}

#[derive(Component)]
pub struct CheckedTile(Entity);

pub fn spawn_checked_tile(
    mut commands: Commands,
    checked_tile_handle: Res<CheckedTileHandle>,
    mut check_ev: EventReader<CheckEvent>,
    tilemap_q: Query<
        (&TilemapGridSize, &TilemapType, &TileStorage, &Transform),
        With<board::BoardTilemap>,
    >,
    tiles_q: Query<Entity, With<CheckedTile>>,
    indicator_q: Query<&CheckedTile>,
) {
    for ev in check_ev.iter() {
        for tile_entity in tiles_q.iter() {
            if let Ok(checked_tile) = indicator_q.get(tile_entity) {
                commands.entity(checked_tile.0).despawn();
                commands.entity(tile_entity).remove::<CheckedTile>();
            }
        }

        let (grid_size, map_type, tile_storage, map_transform) = tilemap_q.single();

        if let Some(check) = ev.0 {
            if let Some(tile_entity) = tile_storage.get(&check) {
                let tile_center = check.center_in_world(grid_size, map_type).extend(0.5);
                let transform = *map_transform * Transform::from_translation(tile_center);

                let checked_tile = commands
                    .spawn(SpriteBundle {
                        texture: checked_tile_handle.0.clone(),
                        transform,
                        ..default()
                    })
                    .id();

                commands
                    .entity(tile_entity)
                    .insert(CheckedTile(checked_tile));
            }
        }
    }
}
