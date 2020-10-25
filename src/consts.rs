use bevy::prelude::*;

pub(crate) const SCALE: f32 = 3.0;
pub(crate) const TILE_SIZE: f32 = 8.0;
pub(crate) const BOUNDING_BOX: Rect<f32> = Rect {
    top: 144.0,
    left: -128.0,
    bottom: -144.0,
    right: 128.0,
};

pub(crate) const TILES_WIDE: usize = 32;
pub(crate) const TILES_HIGH: usize = 36;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Left,
    Right,
    Down,
}
