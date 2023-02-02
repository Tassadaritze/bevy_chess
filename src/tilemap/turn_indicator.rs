use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::IsBlackTurn;
use crate::tilemap::board::BoardTilemap;

#[derive(Component)]
pub struct TurnIndicator(Entity);

pub fn spawn_turn_indicators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap_q: Query<
        (&TilemapGridSize, &TilemapType, &Transform, &TileStorage),
        With<BoardTilemap>,
    >,
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/FreeSansBold.ttf");
    let (grid_size, map_type, tilemap_transform, tilemap_storage) = tilemap_q.single();

    let tile_pos = TilePos { x: 0, y: 0 };
    let tile_entity = tilemap_storage.get(&tile_pos).unwrap();
    let tile_center = tile_pos.center_in_world(grid_size, map_type);

    let transform = *tilemap_transform
        * Transform::from_translation(Vec3::new(tile_center.x - 256., tile_center.y, 2.0));

    let turn_indicator_white = commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "turn",
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 72.,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform,
            ..default()
        })
        .id();

    commands
        .entity(tile_entity)
        .insert(TurnIndicator(turn_indicator_white));

    let tile_pos = TilePos { x: 0, y: 7 };
    let tile_entity = tilemap_storage.get(&tile_pos).unwrap();
    let tile_center = tile_pos.center_in_world(grid_size, map_type);

    let transform = *tilemap_transform
        * Transform::from_translation(Vec3::new(tile_center.x - 256., tile_center.y, 2.0));

    let turn_indicator_black = commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "turn",
                TextStyle {
                    font: font_handle,
                    font_size: 72.,
                    color: *Color::BLACK.as_rgba().set_a(0.3),
                },
            )
            .with_alignment(TextAlignment::CENTER_LEFT),
            transform,
            ..default()
        })
        .id();

    commands
        .entity(tile_entity)
        .insert(TurnIndicator(turn_indicator_black));
}

pub fn draw_turn_indicators(
    is_black_turn: Res<IsBlackTurn>,
    turn_indicator_q: Query<&TurnIndicator>,
    mut colour_q: Query<&mut Text>,
) {
    if is_black_turn.is_changed() {
        for turn_indicator in turn_indicator_q.iter() {
            if let Ok(mut text) = colour_q.get_mut(turn_indicator.0) {
                let colour = &mut text.sections[0].style.color;
                match is_black_turn.0 {
                    true => {
                        if colour.r() > 0. {
                            colour.set_a(0.3);
                        } else {
                            colour.set_a(1.);
                        }
                    }
                    false => {
                        if colour.r() > 0. {
                            colour.set_a(1.);
                        } else {
                            colour.set_a(0.3);
                        }
                    }
                };
            }
        }
    }
}
