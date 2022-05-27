
mod player;
mod mob;
mod main_scene;
mod hud;

use gdnative::prelude::*;

fn init(handle: InitHandle) {
    handle.add_class::<main_scene::MainScene>();
    handle.add_class::<player::Player>();
    handle.add_class::<mob::Mob>();
    handle.add_class::<hud::Hud>();
}
godot_init!(init);
