use godot::{
    classes::{
        AnimationPlayer, CharacterBody2D, ICharacterBody2D, Input, ProjectSettings, Sprite2D,
    },
    global,
    prelude::*,
};

enum State {
    Air,
    Floor,
}

const WALK_SPEED: f32 = 300.0;
const ACCELERATION_SPEED: f32 = WALK_SPEED * 6.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    gravity: f64,
    state: State,
    sprite: OnReady<Gd<Sprite2D>>,
    animation_player: OnReady<Gd<AnimationPlayer>>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            base,
            gravity: 0.0,
            state: State::Air,
            sprite: OnReady::from_node("Sprite2D"),
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
            State::Air => {
                if self.base().is_on_floor() {
                    self.state = State::Floor;
                    return;
                }
                self.walk(&mut velocity, delta);
            }
            State::Floor => {
                self.walk(&mut velocity, delta);
            }
        }

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();

        if !self.base().get_velocity().x.is_zero_approx() {
            let mut scale = self.sprite.get_scale();
            if self.base().get_velocity().x > 0. {
                scale.x = 1.;
            } else {
                scale.x = -1.;
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

impl Player {
    fn walk(&mut self, velocity: &mut Vector2, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("ui_left", "ui_right");
        velocity.x = global::move_toward(
            velocity.x as f64,
            (direction * WALK_SPEED) as f64,
            ACCELERATION_SPEED as f64 * delta,
        ) as f32;
    }

    fn get_new_animation(&self) -> GString {
        if self.base().is_on_floor() {
            return if self.base().get_velocity().abs().x > 0.1 {
                GString::from("run")
            } else {
                GString::from("idle")
            };
        } else {
            return if self.base().get_velocity().y > 0. {
                GString::from("falling")
            } else {
                GString::from("jumping")
            };
        }
    }
}
