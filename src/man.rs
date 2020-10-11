use bevy::prelude::*;
use crate::sprite::Sprite;
use crate::consts::{Direction, SCALE};
use crate::map::{Map, map_tile_to_translation};

pub(crate) struct ManPlugin;
pub(crate) struct PacMan;

impl Plugin for ManPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(setup_man.system())
            .add_system(man_input.system())
        ;
    }
}

fn setup_man(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let pacman_sprite_sheet_components = {
        let texture_handle = asset_server
            .load_sync(
                &mut textures,
                "assets/sprites_trans.png",
            ).unwrap();
        let texture = textures.get(&texture_handle).unwrap();

        let pacman_texture_atlas = TextureAtlas::from_grid(
            texture_handle, texture.size, 14, 13);
        let pacman_texture_atlas_handle = texture_atlases.add(
            pacman_texture_atlas);
        let mut sprite_sheet_components = SpriteSheetComponents {
            texture_atlas: pacman_texture_atlas_handle,
            transform: Transform::from_scale(SCALE)
                .with_translation(map_tile_to_translation(
                    14, 9, Some(0), None)),
            ..Default::default()
        };
        sprite_sheet_components.sprite.index = 1; // First tile, man with open mouth.
        sprite_sheet_components
    };
    commands
        .spawn(pacman_sprite_sheet_components) // Add the man
        .with(Sprite::new(Direction::Right, vec![0, 1, 2])) // Attach sprite infrastructure -- movement and animation
        .with(PacMan)
        .with(Timer::from_seconds(0.04, true))
    ;
}

fn man_input(
    keyboard_input: Res<Input<KeyCode>>,
    map: Res<Map>,
    mut query: Query<(&PacMan, &mut Sprite)>
) {
    // generalize these values for level, etc.
    let speed = 240.0;
    let mut sprite_update: Option<(Vec2, Direction, Vec<u32>)> = None;
    if keyboard_input.pressed(KeyCode::Up) {
        sprite_update =
            Some((
                (0.0, 1.0 * speed).into(),
                Direction::Up,
                vec![28, 29, 2]
            ));
    } else if keyboard_input.pressed(KeyCode::Left) {
        sprite_update =
            Some((
                (-1.0 * speed, 0.0).into(),
                Direction::Left,
                vec![14, 15, 2]
            ));
    } else if keyboard_input.pressed(KeyCode::Down) {
        sprite_update =
            Some((
                (0.0, -1.0 * speed).into(),
                Direction::Down,
                vec![42, 43, 2]
             ));
    } else if keyboard_input.pressed(KeyCode::Right) {
        sprite_update =
            Some((
                 (1.0 * speed, 0.0).into(),
                Direction::Right,
                vec![0, 1, 2],
                 ));
    }
    if let Some((v, d, ix)) = sprite_update {
        println!("Processing direction change...");
        // Checking validity of update.
        for (_pacman, mut sprite) in &mut query.iter() {
            let (x, y) = sprite.tile_info().tile();
            let adjacent = map.get_adjacent(x, y, d);
            if let Some((_, new_tile, _)) = adjacent {
                if new_tile.is_valid_path() {
                    // Handle cornering.. calculate additional factor for velocity.
                    // TODO -- then limit cornering when at position.
                    // this is pacman-specific. The ghosts do not corner.
                    sprite.update(v, d, ix.clone());
                    println!("Okay, should have updated velocity...");
                } else {
                    println!("Cannot change direction: edge of map?")
                }
            } else {
                println!("Cannot change direction: edge of map?")
            }
        }
    }
}
