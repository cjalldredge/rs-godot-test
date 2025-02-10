// use crate::mob;
// use crate::player;
use crate::hud::HUD;
use crate::player::PlayerArea2D;
use crate::mob::MobRigidBody2D;

use godot::prelude::*;
use godot::classes::{Marker2D, RigidBody2D, Timer, PathFollow2D};
use godot::classes::INode;
use godot::classes::notify::NodeNotification;

use rand::Rng;

use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    #[base]
    base: Base<Node>,

    mob_scene: Gd<PackedScene>,

    #[export]
    score: i32,
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        let mut score_timer = self.base().get_node_as::<Timer>("ScoreTimer");
        let mut mob_timer = self.base().get_node_as::<Timer>("MobTimer");

        score_timer.stop();
        mob_timer.stop();

        let mut hud = self.base().get_node_as::<HUD>("HUD");
        let mut hud = hud.bind_mut();
        hud.show_game_over();
    }

    #[func]
    fn new_game(&mut self) {
        godot_print!("New game started.");
        let mut player = self.base().get_node_as::<PlayerArea2D>("Player");
        let start_pos = self.base().get_node_as::<Marker2D>("StartPosition");

        player.bind_mut().start(start_pos.get_position());
        self.base().get_node_as::<Timer>("StartTimer").start();

        let mut my_hud = self.base().get_node_as::<HUD>("HUD");
        let mut my_hud = my_hud.bind_mut();
        my_hud.update_score(self.score.into());
        my_hud.show_message("Get Ready!".into());
    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        let mut hud = self.base_mut().get_node_as::<HUD>("HUD");
        let mut hud = hud.bind_mut();
        hud.update_score(self.score.into());
    }

    #[func]
    fn on_start_timer_timeout(&self) {
        godot_print!("Starting mob and score timers...");

        self.base().get_node_as::<Timer>("MobTimer").start();
        self.base().get_node_as::<Timer>("ScoreTimer").start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        // godot_print!("Spawning new mob...");

        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");

        let mut mob_scene = self.mob_scene.instantiate_as::<RigidBody2D>();

        let mut rng = rand::rng();
        let progress = rng.random_range(u32::MIN..u32::MAX);

        mob_spawn_location.set_progress(progress as f32);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() * PI/2.0;
        direction += rng.random_range(-PI / 4.0..PI / 4.0);

        mob_scene.set_rotation(direction);

        self.base_mut().add_child(&mob_scene);

        let mut mob = mob_scene.cast::<MobRigidBody2D>();
        let range = {
            // Local scope to bind 'mob' user object
            let _mob = mob.bind();
            rng.random_range(150.0..250.0)
        };

        mob.set_linear_velocity(Vector2::new(range, 0.0).rotated(real::from_f32(direction)));
    }
}

#[godot_api]
impl INode for Main {
    fn on_notification(&mut self, _what: NodeNotification) {}

    fn init(base: Base<Node>) -> Self {
        godot_print!("Initializing main scene...");
        Self {
            base,
            mob_scene: PackedScene::new_gd(),
            score: 0,
        }
    }

    fn ready(&mut self) {
        self.mob_scene = load("res://mob_scene.tscn");
        godot_print!("Starting new game...");
        // self.new_game();
    }
}