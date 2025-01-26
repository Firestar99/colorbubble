use crate::delta_time::DeltaTime;
use crate::entity::bubble::Bubble;
use crate::entity::player::Player;
use crate::entity::splash::Splash;
use crate::level::Level;
use std::sync::Arc;
use std::time::Duration;

pub const TIMESTEP: Duration = Duration::from_nanos(16_666_667);

#[derive(Debug, Clone)]
pub struct Game {
    pub level: Arc<Level>,
    pub player: Player,
    pub player_bubble: Option<Bubble>,
    pub particles: Vec<Splash>,
    pub time_sum: Duration,
}

impl Game {
    pub fn new(level: Arc<Level>) -> Self {
        Self {
            player: Player::new(level.entry_point.as_vec2()),
            particles: Vec::new(),
            player_bubble: None,
            level,
            time_sum: Duration::ZERO,
        }
    }

    pub fn update(&mut self, dt: DeltaTime) -> Vec<Splash> {
        self.time_sum += Duration::from_secs_f32(dt.delta_time);
        let mut despawned_particles = Vec::new();

        while let Some(new) = self.time_sum.checked_sub(TIMESTEP) {
            self.time_sum = new;

            if let Some(bubble) = self.player.update(&self.level) {
                if let Some(mut old) = self.player_bubble.replace(bubble) {
                    old.pop(&mut self.particles);
                }
            }
            if let Some(bubble) = &mut self.player_bubble {
                bubble.update(&self.level, &mut self.particles);
                if bubble.dead {
                    self.player_bubble = None;
                }
            }

            let mut remove = Vec::new();
            for (i, particle) in self.particles.iter_mut().enumerate() {
                if particle.update(&self.level) {
                    remove.push(i);
                }
            }

            for i in remove.into_iter().rev() {
                let particle = self.particles.remove(i);
                despawned_particles.push(particle);
            }
        }
        despawned_particles
    }
}
