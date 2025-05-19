use godot::{classes::Camera2D, prelude::*};

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
                let mut camera = child.get_node_as::<Camera2D>("Camera");
                camera.set_limit(Side::LEFT, -315);
                camera.set_limit(Side::TOP, -250);
                camera.set_limit(Side::RIGHT, 955);
                camera.set_limit(Side::BOTTOM, 690);
            }
        });
    }
}
