use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


pub mod player;
pub mod mob;
pub mod hud;
mod main_scene;

// use player::Player;