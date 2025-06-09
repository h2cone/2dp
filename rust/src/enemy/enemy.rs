use godot::{
    classes::{AnimationPlayer, CharacterBody2D, ICharacterBody2D, ProjectSettings, Sprite2D},
    prelude::*,
};

enum State {
    WALK,
    DEAD,
}

const WALK_SPEED: f32 = 22.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Enemy {
    base: Base<CharacterBody2D>,
    state: State,
    sprite: OnReady<Gd<Sprite2D>>,
    gravity: f64,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl ICharacterBody2D for Enemy {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            state: State::WALK,
            sprite: OnReady::from_node("Sprite2D"),
            gravity: 0.0,
            animation_player: OnReady::from_node("AnimationPlayer"),
        }
    }

    fn ready(&mut self) {
        let settings = ProjectSettings::singleton();
        self.gravity = settings.get("physics/2d/default_gravity").to::<f64>();
    }

    fn physics_process(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();
        velocity.y += self.gravity as f32 * delta as f32;

        match self.state {
            State::WALK => {
                if self.base().get_velocity().is_zero_approx() {
                    velocity.x = WALK_SPEED;
                }
                if self.base().is_on_wall() {
                    velocity.x = -velocity.x;
                }
            }
            State::DEAD => {
                todo!();
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        if !self.base().get_velocity().x.is_zero_approx() {
            let mut scale = self.sprite.get_scale();
            if self.base().get_velocity().x > 0.0 {
                scale.x = 0.8;
            } else {
                scale.x = -0.8;
            }
            self.sprite.set_scale(scale);
        }

        let animation = self.get_new_animation();
        if animation != self.animation_player.get_current_animation() {
            self.animation_player.set_current_animation(&animation);
            self.animation_player.play();
        }
    }
}

impl Enemy {
    fn get_new_animation(&self) -> GString {
        if let State::WALK = self.state {
            if self.base().get_velocity().x == 0.0 {
                GString::from("idle")
            } else {
                GString::from("walk")
            }
        } else {
            GString::from("destroy")
        }
    }
}
