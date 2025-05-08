use godot::{classes::InputEvent, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Game {
    base: Base<Node>,
}

#[godot_api]
impl INode for Game {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        godot_print!("Game ready");
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("ui_right") {
            godot_print!("Right key pressed");
        }
    }
}
