use gdnative::prelude::*;
use gdnative::api::{Area2D, AnimatedSprite, PhysicsBody2D, CollisionShape2D, AudioStream, AudioStreamPlayer};
use gdextras::*;
use gdnative::export::hint::{FloatHint, RangeHint};

pub type Base = Area2D;

#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct Player {
    pub speed: f32,
    screen_size: Vector2,
    pub death_sound: Option<GodotString>,
}

impl Player {
    fn new(_owner: TRef<Base>) -> Player {
        Player {
            speed: 400.,
            screen_size: Vector2::ZERO,
            death_sound: None
        }
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.property("speed")
            .with_getter(|s,_| s.speed )
            .with_setter(|mut s,_, v| s.speed = v)
            .with_hint(FloatHint::Range(RangeHint::new(0.1, 1.)))
            .done();

        builder.property("Death sound")
            .with_setter(Self::set_death_sound)
            .done();

        builder.signal("hit")
            .done()
    }

    fn set_death_sound(&mut self, _owner:  TRef<Base>, val: Ref<AudioStream>) {
        let val = unsafe {val.assume_safe()};
        self.death_sound = Some(val.path());
    }
}

#[methods]
impl Player {

    #[export]
    fn _process(&self, owner: TRef<Base>, delta: f32) {
        let mut direction = Vector2::ZERO;
        let input = Input::godot_singleton();

        if input.is_action_pressed("move_up", false) {
            direction += Vector2::UP;
        }
        if input.is_action_pressed("move_right", false) {
            direction += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_down", false) {
            direction += Vector2::DOWN;
        }
        if input.is_action_pressed("move_left", false) {
            direction += Vector2::LEFT;
        }

        let animated_sprite: TRef<AnimatedSprite> = match get_node(owner.clone(), "AnimatedSprite") {
            Ok(sprite) => sprite,
            Err(_) => {
                unsafe {AnimatedSprite::new().into_shared().assume_safe()}
            }
        };

        if direction.length() > 0. {
            animated_sprite.play("", false);
        }
        else {
            animated_sprite.stop();
        }

        if direction.y != 0. {
            animated_sprite.set_animation("up");
            animated_sprite.set_flip_v(direction.y > 0.);
        }
        else if direction.x != 0. {
            animated_sprite.set_flip_v(false);

            animated_sprite.set_animation("walk");
            animated_sprite.set_flip_h(direction.x < 0.);
        }

        if direction.length() > 0. {

            let velocity = Vector2::new((self.screen_size.x * self.speed) * delta, 0.);

            let mut pos = owner.position();
            pos += velocity.rotated(-direction.normalized().angle());

            pos.x = f32::max(0., f32::min(self.screen_size.x, pos.x));
            pos.y = f32::max(0., f32::min(self.screen_size.y, pos.y));

            owner.set_position(pos);
        }

    }


    #[export]
    fn _ready(&mut self, owner: TRef<Base>) {
        self.screen_size = owner.get_viewport_rect().size;

        match self.death_sound.clone() {
            None =>{},
            Some(path) => {
                let loader = ResourceLoader::godot_singleton();
                let audio_res = loader.load(path.clone(), "AudioStream", false).unwrap();
                let audio_res = audio_res.cast::<AudioStream>().unwrap();

                let player = AudioStreamPlayer::new();
                player.set_name("DeathSound");
                player.set_stream(audio_res);

                owner.add_child(player, false);
            }
        }

        owner.hide();
    }

    #[export]
    fn _enter_tree(&self, owner: TRef<Base>) {
        gd_print!(owner, p, "Entering")
    }

    #[export]
    fn _exit_tree(&self, owner: TRef<Base>) {
        gd_print!(owner, p, "Leaving")
    }

    #[export]
    fn _on_player_body_entered(&self, owner: TRef<Base>, _body: Ref<PhysicsBody2D>) {
        owner.hide();

        match self.death_sound {
            None => {},
            Some(_) => {
                get_node::<Base, AudioStreamPlayer>(owner.clone(), "DeathSound").unwrap().play(0.0);
            }
        }


        owner.emit_signal("hit", &[]);
        get_node::<Base, CollisionShape2D>(owner.clone(), "CollisionShape2D").unwrap().set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, owner: TRef<Base>, pos: Vector2) {
        owner.set_position(pos);
        owner.show();
        get_node::<Base, CollisionShape2D>(owner.clone(), "CollisionShape2D").unwrap().set_disabled(false);
    }
}
