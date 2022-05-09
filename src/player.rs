use gdnative::prelude::*;
use gdnative::api::Node2D;
use gdextras::*;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Player;

#[methods]
impl Player {
    fn new(_owner: TRef<Node2D>) -> Player {
        Player
    }


    #[export]
    fn _ready(&self, owner: TRef<Node2D>) {
        gd_print!(owner, p, "Created");
    }

    #[export]
    fn _exit_tree(&self, owner: TRef<Node2D>) {
        gd_print!(owner, p, "Leaving")
    }
}