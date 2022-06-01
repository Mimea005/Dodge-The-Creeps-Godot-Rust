use gdextras::*;
use gdnative::prelude::*;
use gdnative::api::{RigidBody2D, AnimatedSprite};
use rand::prelude::*;


pub type Base = RigidBody2D;
#[derive(NativeClass)]
#[inherit(Base)]
pub struct Mob;


impl Mob {
    fn new(_: TRef<Base>) -> Self {
        Self
    }

    fn get_animations(&self, &owner: &TRef<AnimatedSprite>) -> Vec<String> {
        let frames = owner.sprite_frames().unwrap();
        let frames = unsafe {frames.assume_safe()};

        frames.get_animation_names().to_vec().iter().map(|s|s.to_string()).collect()
    }
}

#[methods]
impl Mob {

    #[export]
    fn _ready(&self, owner: TRef<Base>) {
        let sprite: TRef<AnimatedSprite> = get_node(owner.clone(), "AnimatedSprite").unwrap();

        sprite.set("Playing", true);

        let mob_types = self.get_animations(&sprite);
        if mob_types.len() < 1 {
            gd_print!(owner, e, "no animations present");
        }
        else {
            let mut rng = thread_rng();
            sprite.set_animation(
                mob_types.get(
                    rng.gen_range(0..mob_types.len()
                    )
                )
                    .unwrap());


        }
    }

    #[export]
    fn _on_mob_screen_exited(&self, owner: TRef<Base>) {
        owner.queue_free();
    }

}
