use godot::prelude::*;

mod enemy;
mod game;
mod levels;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
