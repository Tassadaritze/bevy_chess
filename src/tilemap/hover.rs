use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::tilemap::board;
use crate::utils::cursor;

const HOVER_RING_Z: f32 = 3.0;

#[derive(Component)]
pub struct HoverRing;

pub fn spawn_hover_ring(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap: Query<Entity, With<board::BoardTilemap>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("hover_ring.png");

    let ring = commands
        .spawn((
            SpriteBundle {
                visibility: Visibility::INVISIBLE,
                texture: texture_handle,
                transform: Transform {
                    translation: Vec3::Z,
                    ..default()
                },
                ..default()
            },
            HoverRing,
        ))
        .id();

    let tilemap = tilemap.single();
    commands.entity(tilemap).push_children(&[ring]);
}

#[derive(Component)]
pub struct HasHoverRing;

pub fn show_hover_ring(
    mut commands: Commands,
    cursor_pos: Res<cursor::CursorPos>,
    tilemap_q: Query<
        (
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
            &TileStorage,
            &Transform,
        ),
        Without<HoverRing>,
    >,
    hovered_tiles_q: Query<Entity, With<HasHoverRing>>,
    mut hover_ring_q: Query<(&mut Transform, &mut Visibility), With<HoverRing>>,
) {
    for hovered_tile_entity in hovered_tiles_q.iter() {
        let (_, mut hover_ring_visibility) = hover_ring_q.single_mut();
        hover_ring_visibility.is_visible = false;
        commands
            .entity(hovered_tile_entity)
            .remove::<HasHoverRing>();
    }

    for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
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
                let tile_center = tile_pos
                    .center_in_world(grid_size, map_type)
                    .extend(HOVER_RING_Z);
                for (mut hover_ring_transform, mut hover_ring_visibility) in hover_ring_q.iter_mut()
                {
                    *hover_ring_transform = Transform::from_translation(tile_center);
                    hover_ring_visibility.is_visible = true;
                }
                commands.entity(tile_entity).insert(HasHoverRing);
            }
        }
    }
}
