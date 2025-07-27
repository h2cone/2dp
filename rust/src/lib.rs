use godot::prelude::*;

mod enemy;
mod game;
mod level;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
