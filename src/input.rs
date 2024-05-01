use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

#[derive(Resource)]
pub struct InputMap {
    pub left: KeyCode,
    pub right: KeyCode,
    pub up: KeyCode,
    pub down: KeyCode,
    pub attack: KeyCode,
    pub power: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        InputMap {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            attack: KeyCode::Space,
            power: KeyCode::ShiftLeft,
        }
    }
}
//alternative input maps
impl InputMap {
    pub fn alternative_arrows(&mut self) {
        self.left = KeyCode::ArrowLeft;
        self.right = KeyCode::ArrowRight;
        self.up = KeyCode::ArrowUp;
        self.down = KeyCode::ArrowDown;
        self.attack = KeyCode::ControlRight;
        self.power = KeyCode::NumpadEnter;
    }
}

impl InputMap {
    pub fn alternative_numpad(&mut self) {
        self.left = KeyCode::Numpad4;
        self.right = KeyCode::Numpad6;
        self.up = KeyCode::Numpad8;
        self.down = KeyCode::Numpad5;
        self.attack = KeyCode::ShiftRight;
        self.power = KeyCode::NumpadDecimal;
    }
}

#[derive(Resource)]
pub struct Input { //these are ints to track how long a key has been pressed
    pub left: i8,
    pub right: i8,
    pub up: i8,
    pub down: i8,
    pub attack: i8,
    pub power: i8,
    direction: Vec2,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            attack: 0,
            power: 0,
            direction: Vec2::new(0.0, 0.0),
        }
    }
}

impl Input {
    pub fn get_direction(&self) -> Vec2 {
        //normalize the direction vector to int8 max: 127
        let x = self.right - self.left;
        let y = self.up - self.down;
        let mut direction = Vec2::new(x as f32, y as f32);
        if direction.length() > 1.0 {
            direction = direction.normalize();
        }
        direction
    }
}

pub fn get_input(
    input_map: Res<InputMap>,
    mut input: ResMut<Input>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
) {
    //if key is not pressed, set to o. If pressed, increment as long as pressed to a max of 127
    for event in keyboard_input_events.read() {
        match event.state {
            // if key is pressed check its value and save that it was handled
            ButtonState::Pressed => {
                if event.key_code == input_map.left && input.left < 127 {
                    input.left += 1;
                }
                if event.key_code == input_map.right && input.right < 127 {
                    input.right += 1;
                }
                if event.key_code == input_map.up && input.up < 127 {
                    input.up += 1;
                }
                if event.key_code == input_map.down && input.down < 127 {
                    input.down += 1;
                }
                if event.key_code == input_map.attack && input.attack < 127 {
                    input.attack += 1;
                }
                if event.key_code == input_map.power && input.power < 127 {
                    input.power += 1;
                }
            }
            // if key is released, set to 0
            ButtonState::Released => {
                if event.key_code == input_map.left {
                    input.left = 0;
                }
                if event.key_code == input_map.right {
                    input.right = 0;
                }
                if event.key_code == input_map.up {
                    input.up = 0;
                }
                if event.key_code == input_map.down {
                    input.down = 0;
                }
                if event.key_code == input_map.attack {
                    input.attack = 0;
                }
                if event.key_code == input_map.power {
                    input.power = 0;
                }
            }
        }
    }
}