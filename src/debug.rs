use bevy::prelude::*;

use crate::map::{Map, map_tile_to_translation};
use crate::consts::{BOUNDING_BOX, SCALE, TILES_WIDE, TILES_HIGH};

pub(crate) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_grid.system())
        ;
    }
}

fn setup_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    map: Res<Map>
){
    let grid_material = materials.add(Color::rgba(0.4, 0.0, 0.0, 0.4).into());
    // Vertical lines
    for x in 0..TILES_WIDE {
        let start = map_tile_to_translation(x as u32, 0, Some(0), Some(0));
        commands
            .spawn(SpriteComponents {
                material: grid_material,
                transform: Transform::from_translation(
                    Vec3::new(start.x(), 0.0, 1.0)),
                sprite: Sprite::new(Vec2::new(1.0,
                                              (BOUNDING_BOX.top - BOUNDING_BOX.bottom) * SCALE)),
                ..Default::default()
            });
    }
    // Vertical lines
    for y in 0..TILES_HIGH {
        let start = map_tile_to_translation(0, y as u32, Some(0), Some(0));
        commands
            .spawn(SpriteComponents {
                material: grid_material,
                transform: Transform::from_translation(
                    Vec3::new(0.0, start.y(),1.0)),
                sprite: Sprite::new(Vec2::new(
                    (BOUNDING_BOX.right - BOUNDING_BOX.left) * SCALE,
                    1.0)),
                ..Default::default()
            });
    }

    let valid_path_material = materials.add(Color::rgba(0.6, 0.6, 0.6, 0.3).into());
    for x in 0..TILES_WIDE {
        for y in 0..TILES_HIGH {
            let (tile, mut translation) = map.get_tile(x as u32, y as u32);
            translation.set_z(2.0);
            if tile.is_valid_path() {
                commands
                .spawn(SpriteComponents {
                    material: valid_path_material,
                    transform: Transform::from_translation(translation),
                    sprite: Sprite::new(Vec2::new(
                        4.0 * SCALE,
                        4.0 * SCALE)),
                    ..Default::default()
                });
            }
        }
    }
}

