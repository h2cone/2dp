use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct Level00 {
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Level00 {
    fn init(base: Base<Node2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        self.base().get_children().iter_shared().for_each(|child| {
            if child.is_class("Player") {
                godot_print!("Player found");
            }
        });
    }
}
