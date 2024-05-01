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