use std::f64::consts::PI;
use gdnative::{
    prelude::*,
    api::*
};
use gdextras::*;
use gdnative::export::hint::{FloatHint, RangeHint};
use rand::{Rng, rngs::ThreadRng , thread_rng};
use crate::{ hud::Hud, mob, mob::Mob, player, player::Player};


type Base = Node;
#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct MainScene {
    player: String,
    player_speed: f32,
    mob: String,
    hud: String,
    mob_speed_min: f32,
    mob_speed_max: f32,
    start_position: Vector2,
    mob_spawn_interval: f64,
    score_increase_interval: f64,
    start_delay: f64,
    background_music: Option<GodotString>,
    score: i32,
    rng: ThreadRng
}

impl MainScene {
    fn new(_owner: TRef<Base>) -> Self {
        MainScene {
            player: String::new(),
            player_speed: 0.,
            mob: String::new(),
            hud: String::new(),
            start_position: Vector2::ZERO,
            mob_spawn_interval: 0.0,
            score_increase_interval: 0.0,
            start_delay: 0.0,
            background_music: None,
            score: 0,
            rng: thread_rng(),
            mob_speed_min: 100.,
            mob_speed_max: 100.
        }
    }

    fn register(builder: &ClassBuilder<Self>) {

        builder.property("Hud Resource")
            .with_getter(Self::get_hud_res)
            .with_setter(Self::set_hud_res)
            .done();

        builder.property("Background Music")
            .with_setter(Self::set_music)
            .done();

        builder.property("Start Delay")
            .with_setter(|slf, _, val: f64| slf.start_delay = val)
            .with_getter(|slf, _| slf.start_delay)
            .with_hint(FloatHint::Range(RangeHint::new(0.1, 90.)))
            .done();

        builder.property("Score Increase Interval")
            .with_setter(|slf, _, val: f64| slf.score_increase_interval = val)
            .with_getter(|slf, _| slf.score_increase_interval)
            .with_hint(FloatHint::Range(RangeHint::new(0.1, 90.)))
            .done();

        builder.property("Player/Resource")
            .with_getter(Self::get_player_ref)
            .with_setter(Self::set_player_ref)
            .done();

        builder.property("Player/Speed")
            .with_getter(|s,_|s.player_speed)
            .with_setter(|s,_,v|s.player_speed = v)
            .with_hint(FloatHint::Range(RangeHint::new(0.1,1.)))
            .done();

        builder.property("Player/Start Position")
            .with_getter(Self::get_start_pos)
            .with_setter(Self::set_start_pos)
            .done();

        builder.property("Mob/Resource")
            .with_getter(Self::get_mob_ref)
            .with_setter(Self::set_mob_ref)
            .done();

        builder.property("Mob/Mob Spawn Interval")
            .with_setter(|slf, _, val: f64| slf.mob_spawn_interval = val)
            .with_getter(|slf, _| slf.mob_spawn_interval)
            .with_hint(FloatHint::Range(RangeHint::new(0.1, 90.)))
            .done();

        builder.property("Mob/Mob speed min")
            .with_getter(|slf, _| slf.mob_speed_min)
            .with_setter(|slf, _, val:f32| slf.mob_speed_min = val)
            .with_hint(FloatHint::Range(RangeHint::new(50.,400.)))
            .done();

        builder.property("Mob/Mob speed max")
            .with_getter(|slf, _| slf.mob_speed_max)
            .with_setter(|slf, _, val:f32| slf.mob_speed_max = val)
            .with_hint(FloatHint::Range(RangeHint::new(50.,400.)))
            .done();

        builder.signal("show_game_over").done();

        builder.signal("score_update").done();
    }

    fn set_start_pos(&mut self, _owner: TRef<Base>, val: Vector2) {
        self.start_position = val;
    }

    fn get_start_pos(&self, _owner: TRef<Base>) -> Vector2 {
        self.start_position
    }

    fn set_player_ref(&mut self, _owner: TRef<Base>, val: Ref<PackedScene>) {
        let val = unsafe {val.assume_safe()};
        self.player = val.path().to_string();
    }

    fn get_player_ref(&self, owner: TRef<Base>) -> Ref<PackedScene> {
        let loader = ResourceLoader::godot_singleton();
        if loader.exists(self.player.clone(), "") {

            match loader.load(self.player.clone(), "PackedScene", false) {
                Some(res) => {
                    res.cast::<PackedScene>().unwrap()
                },
                None => {
                    gd_print!(owner, e, "Failed to load Player resource");
                    PackedScene::new().into_shared()
                }
            }
        }
        else {
            gd_print!(owner, e, "Player source not defined");
            PackedScene::new().into_shared()
        }
    }

    fn set_mob_ref(&mut self, _owner: TRef<Base>, val: Ref<PackedScene>) {
        let val = unsafe {val.assume_safe()};
        self.mob = val.path().to_string();
    }

    fn get_mob_ref(&self, owner: TRef<Base>) -> Ref<PackedScene> {
        let loader = ResourceLoader::godot_singleton();
        if loader.exists(self.mob.clone(), "") {

            match loader.load(self.mob.clone(), "PackedScene", false) {
                Some(res) => {
                    res.cast::<PackedScene>().unwrap()
                },
                None => {
                    gd_print!(owner, e, "Failed to load Mob resource");
                    PackedScene::new().into_shared()
                }
            }
        }
        else {
            gd_print!(owner, e, "Mob source not defined");
            PackedScene::new().into_shared()
        }
    }

    fn set_hud_res(&mut self, _owner: TRef<Base>, val: Ref<PackedScene>) {
        let val = unsafe {val.assume_safe()};
        self.hud = val.path().to_string();
    }

    fn get_hud_res(&self, owner: TRef<Base>) -> Ref<PackedScene> {
        let loader = ResourceLoader::godot_singleton();
        if loader.exists(self.hud.clone(), "") {
            match loader.load(self.hud.clone(), "PackedScene", false) {
                Some(res) => {
                    res.cast::<PackedScene>().unwrap()
                },
                None => {
                    gd_print!(owner, e, "Failed to load Hud resource");
                    PackedScene::new().into_shared()
                }
            }
        }
        else {
            gd_print!(owner, e, "Hud resource not valid");
            PackedScene::new().into_shared()
        }
    }

    fn set_music(&mut self, _: TRef<Base>, val: Ref<AudioStreamOGGVorbis>) {
        let val = unsafe {val.assume_safe()};
        self.background_music = Some(val.path());
    }

    fn instantiate_player(&self, owner: TRef<Base>) -> Option<Instance<Player, Unique>> {

        let player_res = self.get_player_ref(owner);
        let player_res = unsafe { player_res.assume_safe() };

        if !player_res.can_instance() {gd_print!(owner, e, "Resources not attached")}
        else {
            let node_instance = unsafe {
                player_res.instance(0).unwrap()
                    .assume_unique()
            };

            let instance = node_instance
                .cast::<player::Base>().unwrap()
                .cast_instance::<Player>().unwrap();

            return Some(instance)
        }

        None
    }

    fn instantiate_mob(&self, owner: TRef<Base>) -> Option<Instance<Mob, Unique>> {
        let mob_res = self.get_mob_ref(owner);
        let mob_res = unsafe { mob_res.assume_safe() };

        if !mob_res.can_instance() {gd_print!(owner, e, "Resources not attached")}
        else {
            let node_instance = unsafe {
                mob_res.instance(0).unwrap()
                    .assume_unique()
            };

            let instance = node_instance
                .cast::<mob::Base>().unwrap()
                .cast_instance::<Mob>().unwrap();

            return Some(instance)
        }

        None
    }

    fn setup_fields(&mut self, owner: TRef<Base>) {
        if self.player.is_empty() || self.mob.is_empty()
        {
            gd_print!(owner, e, "Resources not linked");
            panic!();
        }

        //  Timers and mob-spawn area
        {
            let spawn_timer = add_timer("SpawnTimer");
            spawn_timer.set_wait_time(self.mob_spawn_interval);
            owner.add_child(spawn_timer, false);
        }

        {
            let score_timer = add_timer("ScoreTimer");
            score_timer.set_wait_time(self.score_increase_interval);
            owner.add_child(score_timer, false);
        }

        {
            let start_delay = add_timer("StartDelay");
            start_delay.set_wait_time(self.start_delay);
            start_delay.set_one_shot(true);
            owner.add_child(start_delay, false);
        }

        {
            let screen = OS::godot_singleton().window_size();

            let mob_path = Path2D::new();
            mob_path.set_name("MobPath");
            let mob_path_curve = Curve2D::new();

            for point in [
                Vector2::new(0., 0.),
                Vector2::new(0., screen.y),
                Vector2::new(screen.x, screen.y),
                Vector2::new(screen.x, 0.),
                Vector2::new(0., 0.)
            ]
            {
                mob_path_curve.add_point(point, Vector2::ZERO, Vector2::ZERO, -1)
            }
            mob_path.set_curve(mob_path_curve);

            {
                let mob_spawn_location = PathFollow2D::new();
                mob_spawn_location.set_name("MobSpawnLocation");
                mob_spawn_location.set_rotate(true);
                mob_spawn_location.set_loop(true);
                mob_path.add_child(mob_spawn_location, false);

            }

            owner.add_child(mob_path, false);
        }

        // UI
        {
            let background = ColorRect::new();
            background.set_name("Background");
            owner.add_child(background, false);
        }

        {
            let hud_res: Ref<PackedScene> = self.get_hud_res(owner.clone());
            let hud_res: TRef<PackedScene> = unsafe {
                hud_res.assume_safe()
            };
            let hud_inst = unsafe {
                hud_res.instance(0).unwrap().assume_unique()
            };

            let hud_inst = hud_inst.cast::<CanvasLayer>().unwrap()
                .cast_instance::<Hud>().unwrap();

            owner.add_child(hud_inst, false);
        }

        match self.background_music.clone() {
            None => gd_print!(owner, w, "No music registered"),
            Some(path) => {
                let loader = ResourceLoader::godot_singleton();

                if !loader.exists(path.clone(), "") {
                    gd_print!(owner, w, "music source not found at:\n\t{:}", path);
                } else {
                    let music_source = loader.load(path.clone(), "", false).unwrap();
                    let music_source = music_source.cast::<AudioStreamOGGVorbis>().unwrap();

                    let background_music = AudioStreamPlayer::new();
                    background_music.set_name("Background Music");
                    background_music.set_stream(music_source);

                    owner.add_child(background_music, false);
                }
            }
        }


        //  Player
        {
            owner.add_child(
                self.instantiate_player(owner.clone()).unwrap()
                , false);
        }

    }
}

#[methods]
impl MainScene {

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {

        self.setup_fields(owner.clone());

        let background: TRef<ColorRect> = get_node(owner.as_ref(), "Background").unwrap();
        background.set_frame_color(Color::from_hsv(
            240. / 360.,
            0.7,
            0.3
        ));
        background.set_margins_preset(15, 0, 0);
        background.show();

        let player = get_instance::<Node, Area2D, Player>(owner.as_ref(), "Player").unwrap();
        player.map_mut(|s,_|s.speed = self.player_speed).unwrap();

        // Signal connections
        {
            player.base().connect(
                "hit",
                owner.clone(),
                "game_over",
                VariantArray::new().into_shared(),
                0)
                .unwrap();

            get_node::<Base, Timer>(owner.as_ref(), "StartDelay").unwrap()
                .connect(
                    "timeout",
                    owner.clone(),
                    "_on_start_delay_timeout",
                    VariantArray::new().into_shared(),
                    0
                ).unwrap();

            get_node::<Base, Timer>(owner.as_ref(), "ScoreTimer").unwrap()
                .connect(
                    "timeout",
                    owner.clone(),
                    "_on_score_increment",
                    VariantArray::new().into_shared(),
                    0
                ).unwrap();

            get_node::<Base, Timer>(owner.as_ref(), "SpawnTimer").unwrap()
                .connect(
                    "timeout",
                    owner.clone(),
                    "_on_spawn_timer_timeout",
                    VariantArray::new().into_shared(),
                    0
                ).unwrap();

            let hud = get_node::<Base, CanvasLayer>(owner.as_ref(), "Hud").unwrap();

            hud.connect(
                "start_game",
                owner,
                "new_game",
                VariantArray::new().into_shared(),
                0
            ).unwrap();

            owner.connect(
                "score_update",
                hud.clone(),
                "score_update",
                VariantArray::new().into_shared(),
                0
            ).unwrap();

            owner.connect(
                "show_game_over",
                hud.clone(),
                "show_game_over",
                VariantArray::new().into_shared(),
                0
            ).unwrap();
        }

        // self.new_game(owner.clone());
    }

    #[export]
    fn game_over(&self, owner: TRef<Base>) {
        get_node::<Base, Timer>(owner.as_ref(), "SpawnTimer").unwrap().stop();
        get_node::<Base, Timer>(owner.as_ref(), "ScoreTimer").unwrap().stop();
        owner.emit_signal("show_game_over", &[]);

        //Stop music if it exist
        match self.background_music {
            None => {},
            Some(_) => unsafe {
                owner.get_node_as::<AudioStreamPlayer>("Background Music").unwrap().stop()
            }
        }
    }

    #[export]
    fn new_game(&mut self, owner: TRef<Base>) {

        //  Clear mobs not yet despawned from last round
        unsafe {
            owner.get_tree().unwrap().assume_safe().call_group("mobs", "queue_free", &[]);
        }

        self.score = 0;

        let hud_score: TRef<Label> = get_node(owner.as_ref(), "Hud/ScoreLabel").unwrap();
        hud_score.set_text(self.score.to_string());

        let background: TRef<ColorRect> = get_node(owner.as_ref(), "Background").unwrap();
        background.set_frame_color(Color::from_hsv(
            self.rng.gen_range(0_f32..1_f32),
            0.75,
            0.2
        ));

        // If there is background music
            match self.background_music {
                None => {},
                Some(_) => unsafe {
                    owner.get_node_as::<AudioStreamPlayer>("Background Music").unwrap().play(0.0)
                }
            }

        let player = get_instance::<Base, player::Base, Player>(owner.as_ref(), "Player").unwrap();

        player.map(
            |slf, own|
                slf.start(own.as_ref(), self.start_position)
        ).unwrap();

        get_node::<Base, Timer>(owner.as_ref(), "StartDelay").unwrap().start(-1.);

    }

    #[export]
    fn _on_score_increment(&mut self, owner: TRef<Base>) {
        self.score += 1;
        owner.emit_signal("score_update", &[Variant::new(self.score)]);

        if self.score%10 == 0 {
            get_node::<Base, ColorRect>(owner.as_ref(), "Background").unwrap()
                .set_frame_color(Color::from_hsv(
                    self.rng.gen_range(0_f32..1_f32),
                    0.75,
                    0.2
                ));
        }
    }

    #[export]
    fn _on_spawn_timer_timeout(&mut self, owner: TRef<Base>) {
        let mob = self.instantiate_mob(owner.clone()).unwrap();

        let spawn_pos = get_node::<Base, PathFollow2D>(owner.as_ref(), "MobPath/MobSpawnLocation").unwrap();
        spawn_pos.set_unit_offset(self.rng.gen_range(0_f64..1.));

        let mut direction = -spawn_pos.rotation() + PI / 2.;

        mob.base().set_position(spawn_pos.clone().position());

        direction += self.rng.gen_range((-PI/4.)..(PI/4.));
        mob.base().set_rotation(direction);

        let velocity = Vector2::new(self.rng.gen_range(self.mob_speed_min..self.mob_speed_max), 0.);
        mob.base().set_linear_velocity(velocity.rotated(direction as f32));

        // Add a different color to the mob if it has a high speed
        if velocity.length() - self.mob_speed_min > (self.mob_speed_max - self.mob_speed_min) * 0.75 {

            let sprite: TRef<AnimatedSprite> = unsafe {
                mob.base().get_node_as("AnimatedSprite").unwrap()
            };

            let material = CanvasItemMaterial::new();
            material.set_blend_mode(1);
            sprite.set_material(material);
        }

        owner.add_child(mob, false)
    }

    #[export]
    fn _on_start_delay_timeout(&self, owner: TRef<Base>) {
        get_node::<Base, Timer>(owner.as_ref(), "SpawnTimer").unwrap().start(-1.);
        get_node::<Base, Timer>(owner.as_ref(), "ScoreTimer").unwrap().start(-1.);
    }

    #[export]
    fn _input(&self, owner: TRef<Base>, event: Ref<InputEvent>) {
        let event = unsafe {event.assume_safe()};
        if event.is_action("exit_game", false) {
            unsafe {
                owner.get_tree().unwrap().assume_safe()
                    .quit(0);
            }
        }
    }
}

fn add_timer(name: &str) -> Ref<Timer, Unique> {
    let timer = Timer::new();

    timer.set_name(name);

    timer
}
