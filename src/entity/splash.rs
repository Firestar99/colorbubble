use crate::level::Level;
use glam::{Vec2, Vec4, vec2};
use std::f32::consts::E;

const GRAVITY: Vec2 = vec2(0., -0.25);
const DAMP: Vec2 = vec2(1., 1.);
const MAX_AGE: u32 = 100;

#[derive(Debug, Copy, Clone)]
pub struct Splash {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Vec4,
    pub age: u32,
}

impl Splash {
    pub fn spawn_many(particles: &mut Vec<Splash>, pos: Vec2, speed: f32, color: Vec4, n: u32) {
        for i in 0..n {
            particles.push(Splash {
                age: 0,
                pos,
                color,
                vel: Vec2::from_angle(i as f32 * E) * speed,
            });
        }
    }

    // returns true if collided
    pub fn update(&mut self, level: &Level) -> bool {
        self.vel *= DAMP;
        self.vel += GRAVITY;
        self.age += 1;
        self.pos += self.vel;
        level.is_hit(self.pos.as_ivec2()) || self.age > MAX_AGE
    }
}
