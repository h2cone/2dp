use godot::prelude::*;

mod game;
mod level;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
