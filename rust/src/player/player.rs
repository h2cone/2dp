use godot::{
    classes::{
        AnimationPlayer, AudioStreamPlayer2D, CharacterBody2D, ICharacterBody2D, Input,
        ProjectSettings, RayCast2D, Sprite2D, Timer,
    },
    global,
    prelude::*,
};

use super::gun::Gun;

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
    gun: OnReady<Gd<Gun>>,
    shot_animation: OnReady<Gd<Timer>>,
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
            gun: OnReady::from_node("Sprite2D/Gun"),
            shot_animation: OnReady::from_node("ShotAnimation"),
        }
    }

    fn ready(&mut self) {
        let settings = ProjectSettings::singleton();
        self.gravity = settings.get("physics/2d/default_gravity").to::<f64>();
    }

    fn physics_process(&mut self, delta: f64) {
        // Apply gravity
        let mut velocity = self.base().get_velocity();
        velocity.y += self.gravity as f32 * delta as f32;
        // State machine
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
        // Move and slide
        self.base_mut().set_velocity(velocity);
        let detected = self.platform_detector.is_colliding();
        self.base_mut().set_floor_stop_on_slope_enabled(!detected);
        self.base_mut().move_and_slide();
        // Flip
        if !self.base().get_velocity().x.is_zero_approx() {
            let mut scale = self.sprite.get_scale();
            if self.base().get_velocity().x > 0. {
                scale.x = 1.;
            } else {
                scale.x = -1.;
            }
            self.sprite.set_scale(scale);
        }
        // Action
        let mut is_shooting = false;
        if Input::singleton().is_action_just_pressed("shoot") {
            is_shooting = self.gun.bind_mut().try_shoot(self.sprite.get_scale().x);
        }
        // Play animation
        let animation = self.get_new_animation(is_shooting);
        // Will not be interrupted by other actions
        let is_shot_timer_stopped = self.shot_animation.is_stopped();
        if animation != self.animation_player.get_current_animation() && is_shot_timer_stopped {
            if is_shooting {
                self.shot_animation.start();
            }
            self.animation_player.set_current_animation(&animation);
            self.animation_player.play();
        }
    }
}

impl Player {
    fn try_walk(&mut self, velocity: &mut Vector2, delta: f64) {
        let input = Input::singleton();
        let direction = input.get_axis("move_left", "move_right");
        velocity.x = global::move_toward(
            velocity.x as f64,
            (direction * WALK_SPEED) as f64,
            ACCELERATION_SPEED as f64 * delta,
        ) as f32;
    }

    fn try_jump(&mut self, velocity: &mut Vector2) -> bool {
        let input = Input::singleton();
        if input.is_action_just_pressed("jump") {
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

    fn get_new_animation(&self, is_shooting: bool) -> GString {
        let mut animation = if self.base().is_on_floor() {
            if self.base().get_velocity().abs().x > 0.1 {
                "run".to_string()
            } else {
                "idle".to_string()
            }
        } else {
            if self.base().get_velocity().y > 0. {
                "falling".to_string()
            } else {
                "jumping".to_string()
            }
        };
        if is_shooting {
            animation = animation + "_weapon";
        }
        GString::from(animation)
    }
}
