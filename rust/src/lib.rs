use godot::prelude::*;

mod game;
mod levels;
mod player;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
