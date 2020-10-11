use bevy::prelude::*;

use map::MapPlugin;
use man::ManPlugin;
use debug::DebugPlugin;
use sprite::SpritePlugin;

fn main() {
    App::build()
        .add_default_plugins()
        .add_startup_system(global_setup.system())
        .add_plugin(MapPlugin)
//        .add_plugin(DebugPlugin)
        .add_plugin(ManPlugin)
        .add_plugin(SpritePlugin)
        .run();
}

fn global_setup(
    mut commands: Commands,
) {
    commands
        .spawn(Camera2dComponents::default())
    ;
}

mod map;
mod man;
mod sprite;
mod consts;
mod debug;
