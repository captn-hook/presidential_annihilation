use bevy::prelude::*;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct Collider;

pub fn new_platform_bundle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    loc: Vec2,
    size: Vec2,
) -> MaterialMesh2dBundle<ColorMaterial> {
    
    let shape = Mesh2dHandle(meshes.add(Rectangle::new(1.0, 1.0)));

    let color = materials.add(Color::hsl(0.0, 1.0, 0.5));

    let transform = Transform::from_translation(Vec3::new(loc.x, loc.y, 0.0))
        .with_scale(Vec3::new(size.x, size.y, 1.0));

    MaterialMesh2dBundle {
        mesh: shape,
        material: color,
        transform,
        ..default()
    }
}