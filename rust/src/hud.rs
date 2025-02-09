use godot::prelude::*;
use godot::classes::{CanvasLayer, Label, Timer, Button};
use godot::classes::ICanvasLayer;

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl HUD {
    #[signal]
    fn start_game();

    #[func]
    pub fn show_message(&self, text: GString) {
        let mut message_label = self.base().get_node_as::<Label>("Message");
        message_label.set_text(&text);
        message_label.show();

        let mut timer = self.base().get_node_as::<Timer>("Timer");
        timer.start();
    }

    #[func]
    pub fn show_game_over(&self) {
        self.show_message("Game Over".into());

        let mut timer = self.base().get_node_as::<Timer>("Timer");
        timer.connect("timeout", &self.base().callable("show_start_button"));
    }

    #[func]
    pub fn update_score(&mut self, score: i64) {
        self.base().get_node_as::<Label>("ScoreLabel").set_text(&score.to_string());
    }
    
    #[func]
    fn on_start_button_pressed(&mut self) {
        let mut button = self.base().get_node_as::<Button>("StartButton");
        button.hide();

        self.base_mut().emit_signal("start_game", &[]);
    }

    #[func]
    fn on_timer_timeout(&self) {
        let mut message_label = self.base().get_node_as::<Label>("MessageLabel");
        message_label.hide();
    }
}

#[godot_api]
impl ICanvasLayer for HUD {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self {
            base,
        }
    }
}