use bevy::prelude::*;

use crate::map::{
    Map,
    translation_to_map_tile,
    TileInformation,
};
use crate::consts::{Direction, TILES_WIDE, TILE_SIZE, SCALE};

pub(crate) struct Sprite {
    facing: Direction,
    velocity: Vec2,
    texture_indexes: Vec<u32>,
    animating: bool,
    animation_tick: u32,
    tile_info: TileInformation,
}

impl Sprite {
    pub fn update(
        &mut self,
        velocity: Vec2,
        facing: Direction,
        indexes: Vec<u32>) {
        self.velocity = velocity;
        if facing != self.facing {
            self.facing = facing;
            self.texture_indexes = indexes;
            self.animation_tick = 0;
        }
    }

    pub fn new(direction: Direction, indexes: Vec<u32>) -> Self {
        Self {
            facing: direction,
            velocity: Vec2::new(0.0, 0.0),
            texture_indexes: indexes,
            animating: false,
            animation_tick: 0,
            // not a great dummy value, but sprite movement system will run before anything
            // observes this.
            tile_info: TileInformation::default(),
        }
    }

    pub fn update_tile_information(&mut self, tile_info: TileInformation) {
        if tile_info.tile() != self.tile_info.tile() {
            println!("Sprite tile updated to: {:?}", tile_info);
        }
        self.tile_info = tile_info
    }

    pub fn tile_info(&self) -> TileInformation {
        self.tile_info
    }
}

pub(crate) struct SpritePlugin;

impl Plugin for SpritePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(move_sprites.system())
            .add_system(animate_sprites.system())
        ;
    }
}

fn move_sprites(
    time: Res<Time>,
    map: Res<Map>,
    mut query: Query<(&mut Sprite, &mut Transform)>
) {
    for (mut sprite, mut transform) in &mut query.iter() {
        let translation = transform.translation_mut();

        // First, create the proposed new translation.
        let mut new_translation = translation.clone();

        let x_movement = time.delta_seconds * sprite.velocity.x();
        *new_translation.x_mut() += x_movement;

        let y_movement = time.delta_seconds * sprite.velocity.y();
        *new_translation.y_mut() += y_movement;

        // Get the new tile information.
        let new_tile_info = translation_to_map_tile(&new_translation);
        let (x, y) = new_tile_info.tile();
        let maybe_new_tile = map.try_tile(x, y);

        // Have to check if the offset is beyond center, then grab adjacent tile and see if it's valid.
        let is_valid_move = maybe_new_tile.map_or(false, |_| {
            let past_center = new_tile_info.past_center(sprite.facing);
            if past_center {
                let (x, y) = new_tile_info.tile();
                map.get_adjacent(x, y, sprite.facing).map_or(false, |(_, t, _)| t.is_valid_path())
            } else {
                true
            }
        });
        if is_valid_move {
            // Adjust velocity to account for any non-centered offset in tile -- this is "cornering"
            adjust_for_cornering(new_tile_info, sprite.facing, &mut new_translation,
            x_movement, y_movement);

            // Jump tiles if we're teleporting.
            let new_tile_info = tunnel_teleport(new_tile_info, sprite.facing, &mut new_translation);

            // Okay, update the position valid to move into this tile..
            *translation = new_translation;
            sprite.update_tile_information(new_tile_info);
            sprite.animating = true;
        } else {
            // Bumped against a wall. We stop.
            sprite.animating = false;
        }
    }
}

fn adjust_for_cornering(
    tile_info: TileInformation,
    facing: Direction,
    translation: &mut Vec4,
    x_movement: f32,
    y_movement: f32,
) {

    if let Some(tweak) = tile_info.align(facing)  {
        match tweak {
            Direction::Up => {
                assert_eq!(y_movement, 0.0);
                *translation.y_mut() += x_movement.abs();
            },
            Direction::Right => {
                assert_eq!(x_movement, 0.0);
                *translation.x_mut() += y_movement.abs();
            },
            Direction::Down => {
                assert_eq!(y_movement, 0.0);
                *translation.y_mut() += x_movement.abs() * -1.0;
            },
            Direction::Left => {
                assert_eq!(x_movement, 0.0);
                *translation.x_mut() += y_movement.abs() * -1.0;
            },
        }
    }
}

fn tunnel_teleport(
    tile_info: TileInformation,
    facing: Direction,
    translation: &mut Vec4,
) -> TileInformation {
    let teleport_distance = ((TILES_WIDE - 2) as f32 * TILE_SIZE) * SCALE;
    match (facing, tile_info.edge()) {
        (Direction::Right, Some(Direction::Right)) => {
            *translation.x_mut() -= teleport_distance;
            translation_to_map_tile(&translation)
        },
        (Direction::Left, Some(Direction::Left)) => {
            *translation.x_mut() += teleport_distance;
            translation_to_map_tile(&translation)
        },
        _ => tile_info,
    }
}

// TODO  -- cycle through sprite indexes on timer basis
fn animate_sprites(
    mut query: Query<(&mut Timer, &mut Sprite, &mut TextureAtlasSprite)>
) {
    for (timer, mut sprite, mut atlas_sprite) in &mut query.iter() {
        if timer.finished && sprite.animating {
            // Cycle to the next item in the sequence, wrapping.
            sprite.animation_tick =
                (sprite.animation_tick + 1) % (sprite.texture_indexes.len() as u32);
            atlas_sprite.index = sprite.texture_indexes[sprite.animation_tick as usize];
        }
    }
}

