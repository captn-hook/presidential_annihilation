use bevy::prelude::*;

mod input;
mod player;
mod platform;
mod printers;

use input::*;
use player::*;
use platform::*;
use printers::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InputMap::default())
        .insert_resource(Input::default())
        .add_systems(Startup, setup)
        .add_systems(Update, get_input)
        .add_systems(Update, player_movement)
        .add_systems(Update, debug_draw_bounds)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(new_platform_bundle(
        &mut meshes,
        &mut materials,
        Vec2::new(0.0, -100.0),
        Vec2::new(100.0, 10.0),
    )).insert(Collider);

    //sprite bundle
    commands.spawn(new_player_bundle(
        asset_server,
        Vec2::new(0.0, 1.0),
        Vec2::new(20.0, 20.0),
    )).insert(PlayerTransform::default());
}