// use std::str::FromStr;

use godot::prelude::*;
use godot::classes::{AnimatedSprite2D, Area2D, InputMap, PhysicsBody2D, CollisionShape2D};
use godot::classes::IArea2D;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PlayerArea2D {
    #[export]
    speed: f32,
    #[export]
    hide_on_start: bool,
    
    screen_size: Vector2,
    base: Base<Area2D>,
}

#[godot_api]
impl PlayerArea2D {
    #[signal]
    fn hit();

    #[func]
    fn on_player_body_entered(&mut self, _body: Gd<PhysicsBody2D>) {
        self.base_mut().hide();
        self.base_mut().emit_signal("hit", &[]);

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_deferred("disabled", &true.to_variant());
    }

    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_global_position(pos);
        self.base_mut().show();

        let mut collision_shape = self
            .base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D");

        collision_shape.set_disabled(false);
    }
}

#[godot_api]
impl IArea2D for PlayerArea2D {

    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 400.0,
            hide_on_start: true,
            screen_size: Vector2::new(0.0, 0.0),
            base,
        }
    }

    fn ready(&mut self) {
        let viewport = self.base().get_viewport_rect();
        self.screen_size = viewport.size;
        if self.hide_on_start {
            self.base_mut().hide();
        }
    }

    fn process(&mut self, delta: f64) {
        let mut velocity = Vector2::ZERO;
        let input = Input::singleton();
        let input_map = InputMap::singleton();

        if input_map.has_action("move_left") && Input::is_action_pressed(&input, "move_left") {
            velocity.x -= 1.0;
        }
        if input_map.has_action("move_right") && Input::is_action_pressed(&input, "move_right") {
            velocity.x += 1.0;
        }
        if input_map.has_action("move_down") && Input::is_action_pressed(&input, "move_down") {
            velocity.y += 1.0;
        } else if !input_map.has_action("move_down") {
            godot_print!("No action mapped for 'move_down'");
        }
        if input_map.has_action("move_up") && Input::is_action_pressed(&input, "move_up") {
            velocity.y -= 1.0;
        } else if !input_map.has_action("move_up") {
            godot_print!("No action mapped for 'move_up'");
        }

        let change = velocity * self.speed * real::from_f64(delta);
        let position = self.base().get_global_position() + change;
        let position = Vector2::new(
            position.x.clamp(0.0, self.screen_size.x),
            position.y.clamp(0.0, self.screen_size.y),
        );

        velocity = velocity * self.speed;

        let mut animated_sprite_2d = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if velocity.length() > 0.0 {
            // self.base_mut().translate(velocity * delta as f32);
            self.base_mut().set_global_position(position);
            if velocity.x != 0.0 {
                animated_sprite_2d.set_animation("walk");
                animated_sprite_2d.set_flip_h(velocity.x < 0.0);
                animated_sprite_2d.set_flip_v(false);
            } else if velocity.y != 0.0 {
                animated_sprite_2d.set_animation("up");
                animated_sprite_2d.set_flip_v(velocity.y > 0.0);
            }
            animated_sprite_2d.play();
        } else {
            animated_sprite_2d.stop();
        }

    }
}