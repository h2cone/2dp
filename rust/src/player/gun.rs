use godot::{
    classes::{IMarker2D, Marker2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct Gun {
    base: Base<Marker2D>,
}

#[godot_api]
impl IMarker2D for Gun {
    fn init(base: Base<Marker2D>) -> Self {
        Self { base }
    }
}

impl Gun {
    pub fn try_shoot(&self, _direction: f32) -> bool {
        godot_print!("Shooting");
        true
    }
}
