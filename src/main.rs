use bevy::prelude::*;

mod input;
mod player;
mod platform;

use input::*;
use player::*;
use platform::new_platform_bundle;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(InputMap::default())
        .insert_resource(Input::default())
        .add_systems(Startup, setup)
        .add_systems(Update, get_input)
        .add_systems(Update, player_movement)
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
        Vec2::new(0.0, 0.0),
    ));

    //sprite bundle
    commands.spawn(new_player_bundle(
        asset_server,
        Vec2::new(0.0, 1.0),
        Vec2::new(0.5, 0.5),
    )).insert(PlayerTransform::default());
}