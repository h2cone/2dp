use godot::{
    classes::{AnimationPlayer, IRigidBody2D, RigidBody2D, Timer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Bullet {
    base: Base<RigidBody2D>,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl IRigidBody2D for Bullet {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self {
            base,
            animation_player: OnReady::from_node("AnimationPlayer"),
        }
    }

    fn ready(&mut self) {
        let timer = self.base().get_node_as::<Timer>("Timer");
        timer.signals().timeout().connect_other(self, Self::destroy);
    }
}

impl Bullet {
    fn destroy(&mut self) {
        self.animation_player.set_current_animation("destroy");
        self.animation_player.play();
    }
}
