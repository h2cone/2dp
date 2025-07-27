use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Coin {
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Coin {
    fn init(base: Base<Area2D>) -> Self {
        Self { base }
    }
}
