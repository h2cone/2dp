use godot::{
    classes::{AnimationPlayer, IRigidBody2D, RigidBody2D, Timer},
    prelude::*,
};

use crate::enemy::enemy::Enemy;

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

        self.base()
            .signals()
            .body_entered()
            .connect_other(self, Self::on_body_entered);
    }
}

impl Bullet {
    fn destroy(&mut self) {
        self.animation_player.set_current_animation("destroy");
        self.animation_player.play();
    }

    fn on_body_entered(&mut self, body: Gd<Node>) {
        if body.is_class("Enemy") {
            let mut enemy = body.cast::<Enemy>();
            enemy.bind_mut().destroy();
        }
    }
}
