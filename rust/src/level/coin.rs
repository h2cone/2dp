use godot::{
    classes::{AnimationPlayer, Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Coin {
    base: Base<Area2D>,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            base,
            animation_player: OnReady::from_node("AnimationPlayer"),
        }
    }

    fn ready(&mut self) {
        self.base()
            .signals()
            .body_entered()
            .connect_other(self, Self::on_body_entered);
    }
}

impl Coin {
    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        self.animation_player.set_current_animation("picked");
    }
}
