use godot::prelude::*;

struct KitN;

#[gdextension]
unsafe impl ExtensionLibrary for KitN {}

pub mod engine;
pub mod host_godot;
pub mod vm;
