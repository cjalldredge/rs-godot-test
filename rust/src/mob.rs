use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D, AnimatedSprite2D};
// use rand::seq::SliceRandom;
use rand::prelude::*;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct MobRigidBody2D {
    #[base]
    base: Base<RigidBody2D>,
}

#[godot_api]
impl MobRigidBody2D {
    #[func]
    fn on_visibility_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_start_game(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IRigidBody2D for MobRigidBody2D {

    fn init(base: Base<RigidBody2D>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        let mut animated_sprite_2d = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        
        animated_sprite_2d.play();
        let anim_names = animated_sprite_2d.get_sprite_frames().unwrap().get_animation_names();
        
        let anim_names = anim_names.to_vec();
        let mut rng = rand::rng();
        let anim_name = anim_names.choose(&mut rng).unwrap();

        animated_sprite_2d.set_animation(anim_name.arg());
    }
}