use bevy::prelude::*;
use bevy::sprite::SpriteBundle;

use crate::input::*;
use crate::platform::Collider;

#[derive(Component)]
pub struct PlayerTransform {
    pub transform: Transform,
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

pub fn new_player_bundle(
    asset_server: Res<AssetServer>,
    loc: Vec2,
    size: Vec2,
) -> SpriteBundle {

    let texture = asset_server.load("obama.png");
    //get the image out of the texture handle
    
    let transform = Transform::from_translation(Vec3::new(loc.x, loc.y, 0.0))
        .with_scale(Vec3::new(size.x, size.y, 1.0));

    SpriteBundle {
        texture,
        transform,
        ..default()
    }
}

pub fn player_movement(
    mut query: Query<(&mut PlayerTransform, &mut Transform), Without<Collider>>,
    //colliders
    world_query: Query<&Transform, With<Collider>>,
    input: Res<Input>,
    time: Res<Time>,
) {
    for (mut player_transform, mut sprite_transform) in query.iter_mut() {
        let input_dir = input.get_direction();

        let old_velocity = player_transform.velocity * time.delta_seconds() * 10.0;

        let new_velocity = Vec2::new(input_dir.x, input_dir.y) * 300.0 * time.delta_seconds() + old_velocity;

        player_transform.velocity = new_velocity;

        //check for collisions
        for world_transform in world_query.iter() {
            let translation = world_transform.translation;
            let scale = world_transform.scale;

            let player_translation = player_transform.transform.translation;
            let player_scale = player_transform.transform.scale;

            let player_min_x = player_translation.x - player_scale.x / 2.0;
            let player_max_x = player_translation.x + player_scale.x / 2.0;

            let player_min_y = player_translation.y - player_scale.y / 2.0;
            let player_max_y = player_translation.y + player_scale.y / 2.0;

            let world_min_x = translation.x - scale.x / 2.0;
            let world_max_x = translation.x + scale.x / 2.0;

            let world_min_y = translation.y - scale.y / 2.0;
            let world_max_y = translation.y + scale.y / 2.0;

            //if collision is downwards, kill downward velocity
            if player_max_y > world_min_y && player_min_y < world_min_y {
                player_transform.velocity.y = 0.0;
            }
            //else, reflect velocity
            else if player_min_y < world_max_y && player_max_y > world_max_y {
                player_transform.velocity.y = -player_transform.velocity.y;
            }

            //reflect
            if player_min_x < world_max_x && player_max_x > world_max_x {
                player_transform.velocity.x = -player_transform.velocity.x;
            }
            else if player_max_x > world_min_x && player_min_x < world_min_x {
                player_transform.velocity.x = -player_transform.velocity.x;
            }
        }

        //update player position
        let new_pos = player_transform.transform.translation + Vec3::new(new_velocity.x, new_velocity.y, 0.0);
        player_transform.transform.translation = new_pos;
        //set entity transform to new transform
        sprite_transform.translation = player_transform.transform.translation;
    }
}