use godot::{
    classes::{
        AnimationPlayer, AudioStreamPlayer2D, CharacterBody2D, ICharacterBody2D, Input,
        ProjectSettings, RayCast2D, Sprite2D,
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
const JUMP_VELOCITY: f32 = -339.0;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    base: Base<CharacterBody2D>,
    gravity: f64,
    state: State,
    sprite: OnReady<Gd<Sprite2D>>,
    animation_player: OnReady<Gd<AnimationPlayer>>,
    jump_sound: OnReady<Gd<AudioStreamPlayer2D>>,
    platform_detector: OnReady<Gd<RayCast2D>>,
    double_jump_charged: bool,
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
            jump_sound: OnReady::from_node("JumpSound"),
            platform_detector: OnReady::from_node("PlatformDetector"),
            double_jump_charged: false,
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
                    self.double_jump_charged = true;
                    return;
                }
                self.try_walk(&mut velocity, delta);
                self.try_jump(&mut velocity);
            }
            State::Floor => {
                self.try_walk(&mut velocity, delta);
                if self.try_jump(&mut velocity) {
                    self.state = State::Air;
                }
            }
        }

        self.base_mut().set_velocity(velocity);
        let detected = self.platform_detector.is_colliding();
        self.base_mut().set_floor_stop_on_slope_enabled(!detected);
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
    fn try_walk(&mut self, velocity: &mut Vector2, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("ui_left", "ui_right");
        velocity.x = global::move_toward(
            velocity.x as f64,
            (direction * WALK_SPEED) as f64,
            ACCELERATION_SPEED as f64 * delta,
        ) as f32;
    }

    fn try_jump(&mut self, velocity: &mut Vector2) -> bool {
        let input = Input::singleton();
        if input.is_action_just_pressed("ui_up") {
            if self.base().is_on_floor() {
                velocity.y = JUMP_VELOCITY;
                self.jump_sound.set_pitch_scale(1.);
            } else if self.double_jump_charged {
                velocity.y = JUMP_VELOCITY;
                self.double_jump_charged = false;
                self.jump_sound.set_pitch_scale(1.5);
            } else {
                return false;
            }
            self.jump_sound.play();
            return true;
        }
        false
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
