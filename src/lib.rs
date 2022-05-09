mod main_scene;
mod player;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<main_scene::MainScene>();
    handle.add_class::<player::Player>();
}
godot_init!(init);