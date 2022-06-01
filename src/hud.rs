use gdextras::get_node;
use gdnative::prelude::*;
use gdnative::api::CanvasLayer;

pub type Base = CanvasLayer;
#[derive(NativeClass)]
#[inherit(Base)]
#[register_with(Self::register)]
pub struct Hud;

impl Hud {
    fn new(_:TRef<Base>) -> Self {
        Self
    }

    fn register(builder: &ClassBuilder<Self>) {
        builder.signal("start_game")
            .done();
    }
}

#[methods]
impl Hud {

    #[export]
    fn _on_get_ready_timeout(&self, owner: TRef<Base>) {
        let message: TRef<Label> = get_node(owner.clone(), "Message").unwrap();
        message.hide();
    }

    #[export]
    fn _on_game_over_timeout(&self, owner: TRef<Base>) {
        let message: TRef<Label> = get_node(owner.clone(), "Message").unwrap();
        message.set_text("Dodge the Creeps!");
        message.show();
        get_node::<Base, Button>(owner.clone(), "StartButton").unwrap().show();
    }

    #[export]
    fn _on_start_button_pressed(&self, owner: TRef<Base>) {
        get_node::<Base, Label>(owner.clone(), "Message").unwrap().set_text("Get Ready");
        get_node::<Base, Timer>(owner.clone(), "GetReadyTimer").unwrap().start(-1.);
        get_node::<Base, Button>(owner.clone(), "StartButton").unwrap()
            .hide();
        owner.emit_signal("start_game", &[]);
    }

    #[export]
    fn show_game_over(&self, owner: TRef<Base>) {
        let message: TRef<Label> = get_node(owner.clone(), "Message").unwrap();
        message.set_text("Game Over");
        message.show();

        get_node::<Base, Timer>(owner.clone(), "GameOverTimer").unwrap()
            .start(-1.);
    }

    #[export]
    fn score_update(&self, owner: TRef<Base>, score: i32) {
        get_node::<Base, Label>(owner.clone(), "ScoreLabel").unwrap().set_text(score.to_string());
    }
}
