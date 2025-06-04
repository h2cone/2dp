use godot::{
    classes::{AudioStreamPlayer2D, IMarker2D, Marker2D, Timer},
    prelude::*,
};

use crate::player::bullet::Bullet;

const BULLET_SPEED: f32 = 850.0;

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct Gun {
    base: Base<Marker2D>,
    cooldown: OnReady<Gd<Timer>>,
    bullet_scene: Option<Gd<PackedScene>>,
    shoot_sound: OnReady<Gd<AudioStreamPlayer2D>>,
}

#[godot_api]
impl IMarker2D for Gun {
    fn init(base: Base<Marker2D>) -> Self {
        Self {
            base,
            cooldown: OnReady::from_node("Cooldown"),
            bullet_scene: None,
            shoot_sound: OnReady::from_node("ShootSound"),
        }
    }

    fn ready(&mut self) {
        self.bullet_scene = Some(
            try_load::<PackedScene>("res://player/bullet.tscn")
                .expect("Failed to load bullet scene"),
        );
    }
}

impl Gun {
    pub fn try_shoot(&mut self, direction: f32) -> bool {
        if !self.cooldown.is_stopped() {
            return false;
        }
        let mut bullet = self
            .bullet_scene
            .as_ref()
            .unwrap()
            .instantiate_as::<Bullet>();
        bullet.set_global_position(self.base().get_global_position());
        bullet.set_linear_velocity(Vector2::new(direction * BULLET_SPEED, 0.));
        // If true, this CanvasItem will not inherit its transform from parent CanvasItems.
        bullet.set_as_top_level(true);
        self.base_mut().add_child(&bullet);
        self.shoot_sound.play();
        self.cooldown.start();
        true
    }
}
