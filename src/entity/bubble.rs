use crate::entity::splash::Particle;
use crate::level::Level;
use glam::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct Bubble {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dead: bool,
}

impl Bubble {
    pub fn update(&mut self, level: &Level, particles: &mut Vec<Particle>) -> bool {
        if self.dead {
            return self.dead;
        }

        let new_pos = self.pos + self.vel;
        if level.is_hit(new_pos.as_uvec2()) {
            self.pop(particles);
        } else {
            self.pos = new_pos;
        }

        self.dead
    }

    pub fn pop(&mut self, particles: &mut Vec<Particle>) {
        self.dead = true;
        for i in 0..10 {
            particles.push(Particle {
                age: 0,
                pos: self.pos,
                vel: Vec2::from_angle(i as f32) * 5.0,
            });
        }
    }
}
