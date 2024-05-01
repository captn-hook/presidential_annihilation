use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

pub fn print_window(window: Query<&Window>) {
    let window = window.single();

    let width = window.resolution.width();
    let height = window.resolution.height();

    let (x, y) = match window.position {
        WindowPosition::At(v) => (v.x as f32, v.y as f32),
        _ => (0., 0.),
    };

    dbg!(width, height, x, y);
}

pub fn print_keypress(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}

pub fn debug_draw_bounds(
    mut gizmos: Gizmos,
    transforms: Query<&Transform>,
) {
    for transform in transforms.iter() {
        let translation = transform.translation;
        let scale = transform.scale;

        gizmos.rect_2d(
            Vec2::new(translation.x, translation.y),
            0.0,
            Vec2::new(scale.x, scale.y),
            Color::WHITE,
        );
    }
}