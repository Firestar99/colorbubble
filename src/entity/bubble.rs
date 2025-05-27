use crate::entity::splash::Splash;
use crate::level::Level;
use glam::{Vec2, Vec4, vec2};

const GRAVITY: Vec2 = vec2(0., 0.1);
const DAMP: Vec2 = vec2(0.95, 0.95);

#[derive(Debug, Copy, Clone, Default)]
pub struct Bubble {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Vec4,
    pub dead: bool,
}

impl Bubble {
    pub fn update(&mut self, level: &Level, particles: &mut Vec<Splash>) {
        self.vel *= DAMP;
        self.vel += GRAVITY;
        let new_pos = self.pos + self.vel;
        if level.is_hit(new_pos.as_ivec2()) {
            self.pop(particles);
        } else {
            self.pos = new_pos;
        }
    }

    pub fn pop(&mut self, particles: &mut Vec<Splash>) {
        if self.dead {
            return;
        }
        Splash::spawn_many(particles, self.pos, 1., self.color, 10);

        self.dead = true;
    }
}
