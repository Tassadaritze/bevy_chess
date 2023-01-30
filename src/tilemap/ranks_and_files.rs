use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
struct RankFileLabel(Entity);

pub fn create_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tilemap_q: Query<
        (&TilemapGridSize, &TilemapType, &Transform, &TileStorage),
        With<super::board::BoardTilemap>,
    >,
    tile_q: Query<&TilePos>,
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/UbuntuMono-R.ttf");
    let (grid_size, map_type, tilemap_transform, tilemap_storage) = tilemap_q.single();
    for tile_entity in tilemap_storage.iter().flatten() {
        let tile_pos = tile_q.get(*tile_entity).unwrap();
        if (1..7).contains(&tile_pos.x) && (1..7).contains(&tile_pos.y) {
            continue;
        }
        let tile_center = tile_pos.center_in_world(grid_size, map_type);

        let text = get_label_for_pos(
            tile_pos,
            *tilemap_transform,
            tile_center,
            font_handle.clone(),
        );

        let label = commands.spawn(text.0).id();
        commands.entity(*tile_entity).insert(RankFileLabel(label));

        if let Some(text) = text.1 {
            let label = commands.spawn(text).id();
            commands.entity(*tile_entity).insert(RankFileLabel(label));
        }
    }
}

fn get_label_for_pos(
    tile_pos: &TilePos,
    tilemap_transform: Transform,
    tile_center: Vec2,
    font_handle: Handle<Font>,
) -> (Text2dBundle, Option<Text2dBundle>) {
    if [0, 7].contains(&tile_pos.x) && [0, 7].contains(&tile_pos.y) {
        (
            get_file_label(
                tile_pos,
                tilemap_transform,
                tile_center,
                font_handle.clone(),
            ),
            Some(get_rank_label(
                tile_pos,
                tilemap_transform,
                tile_center,
                font_handle,
            )),
        )
    } else if [0, 7].contains(&tile_pos.y) {
        (
            get_file_label(tile_pos, tilemap_transform, tile_center, font_handle),
            None,
        )
    } else {
        (
            get_rank_label(tile_pos, tilemap_transform, tile_center, font_handle),
            None,
        )
    }
}

fn get_file_label(
    tile_pos: &TilePos,
    tilemap_transform: Transform,
    tile_center: Vec2,
    font_handle: Handle<Font>,
) -> Text2dBundle {
    let text_section = match tile_pos.x {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("tried to get file label for x: {}", tile_pos.x),
    };

    let alignment = match tile_pos.y {
        0 => TextAlignment::BOTTOM_CENTER,
        7 => TextAlignment::TOP_CENTER,
        _ => panic!("tried to get file label alignment for y: {}", tile_pos.y),
    };

    let offset = match tile_pos.y {
        0 => -100.0,
        7 => 100.0,
        _ => panic!("tried to get file label offset for y: {}", tile_pos.y),
    };
    let transform = tilemap_transform
        * Transform::from_translation(Vec3::new(tile_center.x, tile_center.y + offset, 2.0));

    Text2dBundle {
        text: Text::from_section(
            text_section,
            TextStyle {
                font: font_handle,
                font_size: 48.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(alignment),
        transform,
        ..default()
    }
}

fn get_rank_label(
    tile_pos: &TilePos,
    tilemap_transform: Transform,
    tile_center: Vec2,
    font_handle: Handle<Font>,
) -> Text2dBundle {
    let text_section = match tile_pos.y {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => panic!("tried to get rank label for y: {}", tile_pos.y),
    };

    let alignment = match tile_pos.x {
        0 => TextAlignment::CENTER_LEFT,
        7 => TextAlignment::CENTER_RIGHT,
        _ => panic!("tried to get rank label alignment for x: {}", tile_pos.x),
    };

    let offset = match tile_pos.x {
        0 => -100.0,
        7 => 100.0,
        _ => panic!("tried to get rank label offset for x: {}", tile_pos.x),
    };
    let transform = tilemap_transform
        * Transform::from_translation(Vec3::new(tile_center.x + offset, tile_center.y, 2.0));

    Text2dBundle {
        text: Text::from_section(
            text_section,
            TextStyle {
                font: font_handle,
                font_size: 48.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(alignment),
        transform,
        ..default()
    }
}
