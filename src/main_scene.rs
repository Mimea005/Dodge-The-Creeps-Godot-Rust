use gdnative::prelude::*;
use gdnative::api::{Node};
use gdextras::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_class)]
pub struct MainScene {
    pub player: GodotString,
    pub enemy: GodotString
}

#[allow(unused_variables)]
impl MainScene {

    fn register_class(build: &ClassBuilder<Self>) {
        build.property("Player")
            .with_getter(Self::get_player)
            .with_setter(Self::set_player)
            .done();
        build.property("Enemy")
            .with_getter(Self::get_enemy)
            .with_setter(Self::set_enemy)
            .done();
    }

    fn set_player(&mut self, owner: TRef<Node>, scene: Ref<PackedScene>) {
        unsafe { self.player = scene.assume_safe().path() }
    }

    fn get_player(&self, owner: TRef<Node>) -> Ref<PackedScene> {

        ResourceLoader::godot_singleton()
            .load(self.player.clone(), "PackedScene", false)
            .unwrap()
            .cast::<PackedScene>()
            .unwrap()
    }

    fn set_enemy(&mut self, owner: TRef<Node>, scene: Ref<PackedScene>) {
        unsafe { self.player = scene.assume_safe().path() }
    }

    fn get_enemy(&self, owner: TRef<Node>) -> Ref<PackedScene> {

        ResourceLoader::godot_singleton()
            .load(self.enemy.clone(), "PackedScene", false)
            .unwrap()
            .cast::<PackedScene>()
            .unwrap()
    }
}

#[methods]
impl MainScene {

    fn new(_owner: TRef<Node>) -> MainScene {
        MainScene{
            player: GodotString::new(),
            enemy: GodotString::new()
        }
    }


    #[export]
    fn _enter_tree(&self, owner: TRef<Node>) {
        gd_print!(owner, p, "Entering");
    }

    #[export]
    fn _ready(&self, owner: TRef<Node>) {

        let player_scene = ResourceLoader::godot_singleton()
            .load(self.player.clone(), "PackedScene", false)
            .unwrap()
            .cast::<PackedScene>()
            .unwrap();

        let player_scene = unsafe { player_scene.assume_safe() };

        gd_print!(owner, p, "Resource loaded {:?}", player_scene.path());

        let player = player_scene.instance(0).unwrap();
        let player =  unsafe { player.assume_unique().cast::<Node2D>().unwrap() };

        owner.add_child(player, false);

    }

    #[export]
    fn _exit_tree(&self, owner: TRef<Node>) {
        gd_print!(owner, p, "Leaving")
    }
}