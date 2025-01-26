use crate::level::Level;
use glam::{Vec2, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub age: usize,
}

impl Particle {
    // returns true if collided
    pub fn update(&mut self, level: &Level) -> bool {
        self.age += 1;
        self.pos += self.vel;
        level.is_hit(self.pos.as_uvec2()) || self.age > 500
    }
}

pub struct DespawnedParticle {
    pub pos: Vec2,
    pub color: Vec3,
}
