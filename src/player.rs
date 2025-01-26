use macroquad::math::{vec2, Vec2};
use macroquad_platformer::Actor;

pub struct Player {
    pub collider: Actor,
    pub speed: Vec2,
    pub facing_left: bool,
}

impl Player {
    pub fn new(collider: Actor) -> Self {
        Player {
            collider,
            speed: vec2(0., 0.),
            facing_left: false,
        }
    }
}
