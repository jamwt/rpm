use bevy::prelude::*;
use smallvec::SmallVec;
use ndarray::{Array, Array2};

use crate::consts::{BOUNDING_BOX, SCALE, TILE_SIZE, Direction};

const I: bool = false;
const X: bool = true;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) struct TileInformation(u32, u32, u32, u32);

impl TileInformation {
    pub fn new() -> Self {
        Self(0, 0, 0, 0)
    }

    pub fn x(&self) -> u32 {
        self.0
    }

    pub fn y(&self) -> u32 {
        self.1
    }

    pub fn x_offset(&self) -> u32 {
        self.2
    }

    pub fn y_offset(&self) -> u32 {
        self.3
    }

    pub fn tile(&self) -> (u32, u32) {
        (self.x(), self.y())
    }

    pub fn past_center(&self, facing: Direction) -> bool {
        let center = TILE_SIZE as u32 / 2;
        match facing {
            Direction::Up => {
                self.y_offset() >= center
            },
            Direction::Right => {
                self.x_offset() >= center
            },
            Direction::Down => {
                self.y_offset() < center
            },
            Direction::Left => {
                self.x_offset() < center
            },
        }
    }

    pub fn align(&self, facing: Direction) -> Option<Direction> {
        let center = TILE_SIZE as u32 / 2;
        match facing {
            Direction::Up | Direction::Down => {
                if self.x_offset() == center {
                    None
                } else if self.x_offset() > center {
                    Some(Direction::Left)
                } else {
                    Some(Direction::Right)
                }
            },
            Direction::Left | Direction::Right => {
                if self.y_offset() == center {
                    None
                } else if self.y_offset() > center {
                    Some(Direction::Down)
                } else {
                    Some(Direction::Up)
                }
            },
        }
    }
}

impl From<(u32, u32, u32, u32)> for TileInformation {
    fn from(inp: (u32, u32, u32, u32)) -> Self {
        Self(inp.0, inp.1, inp.2, inp.3)
    }
}

impl Default for TileInformation {
    fn default() -> Self {
        Self(0, 0, 0, 0)
    }
}

static MAP_PATH_VALIDITY: &[bool] = &[
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, X, X, X, X, X, X, X, X, X, X, X, X, I, I, X, X, X, X, X, X, X, X, X, X, X, X, I,
    I, X, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, X, I,
    I, X, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, X, I,
    I, X, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, X, I,
    I, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, I,
    I, X, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, X, I,
    I, X, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, X, I,
    I, X, X, X, X, X, X, I, I, X, X, X, X, I, I, X, X, X, X, I, I, X, X, X, X, X, X, I,
    I, I, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, X, X, X, X, X, X, X, X, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    X, X, X, X, X, X, X, X, X, X, I, I, I, I, I, I, I, I, X, X, X, X, X, X, X, X, X, X,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, X, X, X, X, X, X, X, X, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I,
    I, X, X, X, X, X, X, X, X, X, X, X, X, I, I, X, X, X, X, X, X, X, X, X, X, X, X, I,
    I, X, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, X, I,
    I, X, I, I, I, I, X, I, I, I, I, I, X, I, I, X, I, I, I, I, I, X, I, I, I, I, X, I,
    I, X, X, X, I, I, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, I, I, X, X, X, I,
    I, I, I, X, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, X, I, I, I,
    I, I, I, X, I, I, X, I, I, X, I, I, I, I, I, I, I, I, X, I, I, X, I, I, X, I, I, I,
    I, X, X, X, X, X, X, I, I, X, X, X, X, I, I, X, X, X, X, I, I, X, X, X, X, X, X, I,
    I, X, I, I, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, I, I, X, I,
    I, X, I, I, I, I, I, I, I, I, I, I, X, I, I, X, I, I, I, I, I, I, I, I, I, I, X, I,
    I, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, X, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
    I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I, I,
];

pub(crate) struct Map {
    tiles: Array2<Tile>,
}

impl Map {
    pub fn get_tile(&self, x: u32, y:u32) -> (Tile, Vec3) {
        self.try_tile(x, y).expect("bad tile coords?")
    }

    pub fn try_tile(&self, x: u32, y:u32) -> Option<(Tile, Vec3)> {
        self.tiles
            .get((x as usize, y as usize)).map(|tile| {
            let translation = map_tile_to_translation(x, y, None, None);
            (tile.clone(), translation)
        })
    }

    pub fn get_adjacent(&self,
                        mut x: u32,
                        mut y: u32,
                        direction: Direction) -> Option<((u32, u32), Tile, Vec3)> {
        match direction {
            Direction::Up => {
                if y == 35 {
                    return None;
                } else {
                    y += 1;
                }
            },
            Direction::Right => {
                if x == 27 {
                    return None;
                } else {
                    x += 1;
                }
            },
            Direction::Down => {
                if y == 0 {
                    return None;
                } else {
                    y -= 1;
                }
            },
            Direction::Left => {
                if x == 0 {
                    return None;
                } else {
                    x -= 1;
                }
            },
        }
        let (tile, translation) = self.get_tile(x, y);
        Some(((x, y), tile, translation))
    }
}

impl Default for Map {
    fn default() -> Self {
        assert_eq!(MAP_PATH_VALIDITY.len(), 28 * 36);
        Self {
            tiles: Array::from_shape_fn((28, 36),
                                        |(i, j)|  {
                                            let idx = ((35 - j) * 28)+i;
                                            if MAP_PATH_VALIDITY[idx] {
                                                Tile::Path(PathTile::default())
                                            } else {
                                                Tile::Invalid
                                            }
                                        })
        }
    }
}

#[derive(Clone)]
pub(crate) enum Tile {
    Invalid,
    Path(PathTile),
}

impl Tile {
    pub fn is_valid_path(&self) -> bool {
        match self {
            &Tile::Path(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub(crate) enum TileActor {
    Ghost,
    Bonus,
}

#[derive(Default, Clone)]
pub(crate) struct PathTile {
    contents: SmallVec<[TileActor; 8]>,
}

pub(crate) struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let map = Map::default();
        app
            .add_resource(map)
            .add_startup_system(setup_map.system())
        ;
    }
}

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map_sprite_sheet_components = {
        let texture_handle = asset_server
            .load_sync(
                &mut textures,
                "assets/map.png",
            ).unwrap();
        let texture = textures.get(&texture_handle).unwrap();

        // Let's set up the map.
        let map_texture_atlas = TextureAtlas::from_grid(
            texture_handle, texture.size, 1, 1);
        let map_texture_atlas_handle = texture_atlases.add(map_texture_atlas);
        let sprite_sheet_components = SpriteSheetComponents {
            texture_atlas: map_texture_atlas_handle,
            transform: Transform::from_scale(SCALE)
                .with_translation(Vec3::new(0.0, -4.0 * SCALE, 0.0)),
            ..Default::default()
        };
        sprite_sheet_components
    };
    commands
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
//           transform: Transform::from_translation(Vec3::new(BOUNDING_BOX.left * SCALE,
//                                                           BOUNDING_BOX.top * SCALE, 0.0)),
            sprite: Sprite::new(Vec2::new((BOUNDING_BOX.right - BOUNDING_BOX.left) * SCALE,
                                          (BOUNDING_BOX.top - BOUNDING_BOX.bottom) * SCALE)),
            ..Default::default()
        })
        .spawn(map_sprite_sheet_components) // Add the map.
    ;

}

pub(crate) fn map_tile_to_translation(
    x_tile: u32,
    y_tile: u32,
    x_offset: Option<u32>,
    y_offset: Option<u32>) -> Vec3 {

    let mut x_pos = BOUNDING_BOX.left;
    let mut y_pos = BOUNDING_BOX.bottom;

    x_pos += (x_tile as f32) * TILE_SIZE;
    y_pos += (y_tile as f32) * TILE_SIZE;

    if let Some(x) = x_offset {
        x_pos += x as f32;
    } else {
        x_pos += TILE_SIZE / 2.0;
    }

    if let Some(y) = y_offset {
        y_pos += y as f32;
    } else {
        y_pos += TILE_SIZE / 2.0;
    }

    let res = Vec3::new(x_pos * SCALE, y_pos * SCALE, 0.0);
    res
}

pub(crate) fn translation_to_map_tile(
    translation: &Vec4,
) -> TileInformation {

    let x = (translation.x() / SCALE) + BOUNDING_BOX.right;
    let x_tile = x as u32 / TILE_SIZE as u32;
    let x_offset = x as u32 % TILE_SIZE as u32;

    let y = (translation.y() / SCALE) + BOUNDING_BOX.top;
    let y_tile = y as u32 / TILE_SIZE as u32;
    let y_offset = y as u32 % TILE_SIZE as u32;

    (x_tile, y_tile, x_offset, y_offset).into()
}
