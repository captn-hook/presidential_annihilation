use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::input::*;

#[derive(Component)]
pub struct PlayerTransform {
    transform: Transform,
    pub velocity: Vec2,
}

impl Default for PlayerTransform {
    fn default() -> Self {
        PlayerTransform {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            velocity: Vec2::new(0.0, 0.0),
        }
    }
}

impl PlayerTransform {
    pub fn update_transform(&mut self) {
        //add velocity to transform
        let new_translation = self.transform.translation + Vec3::new(self.velocity.x, self.velocity.y, 0.0);
        self.transform = Transform::from_translation(new_translation);
    }

    pub fn get_transform(&self) -> Transform {
        self.transform
    }
}

pub fn new_player_bundle(
    asset_server: Res<AssetServer>,
    loc: Vec2,
    size: Vec2,
) -> SpriteBundle {

    let transform = Transform::from_translation(Vec3::new(loc.x, loc.y, 0.0))
        .with_scale(Vec3::new(size.x, size.y, 1.0));

    SpriteBundle {
        texture: asset_server.load("obama.png"),
        transform,
        ..default()
    }
}

pub fn player_movement(
    mut query: Query<(&mut PlayerTransform, &mut Transform)>,
    input: Res<Input>,
    time: Res<Time>,
) {
    for (mut player_transform, mut sprite_transform) in query.iter_mut() {
        let input_dir = input.get_direction();

        let old_velocity = player_transform.velocity * time.delta_seconds();

        let new_velocity = Vec2::new(input_dir.x, input_dir.y) * 100.0 * time.delta_seconds() + old_velocity;

        player_transform.velocity = new_velocity;

        player_transform.update_transform();

        //set entity transform to new transform
        sprite_transform.translation = player_transform.get_transform().translation;
    }
}