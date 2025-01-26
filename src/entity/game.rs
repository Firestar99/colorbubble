use crate::delta_time::DeltaTime;
use crate::entity::bubble::Bubble;
use crate::entity::player::Player;
use crate::entity::portal::Portal;
use crate::entity::splash::Splash;
use crate::level::Level;
use std::sync::Arc;
use std::time::Duration;

pub const TIMESTEP: Duration = Duration::from_nanos(16_666_667);

#[derive(Debug, Clone)]
pub struct Game {
    pub level: Arc<Level>,
    pub player: Player,
    pub portal: Portal,
    pub player_bubble: Option<Bubble>,
    pub splashes: Vec<Splash>,
    pub time_sum: Duration,
}

impl Game {
    pub fn new(level: Arc<Level>) -> Self {
        Self {
            player: Player::new(level.entry_point.as_vec2()),
            portal: Portal::new(level.portal.as_vec2()),
            splashes: Vec::new(),
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

            if let Some(bubble) = self.player.update(&self.level, &mut self.splashes) {
                if let Some(mut old) = self.player_bubble.replace(bubble) {
                    old.pop(&mut self.splashes);
                }
            }
            if let Some(bubble) = &mut self.player_bubble {
                bubble.update(&self.level, &mut self.splashes);
                if bubble.dead {
                    self.player_bubble = None;
                }
            }

            self.portal.update(&mut self.player);

            let mut remove = Vec::new();
            for (i, particle) in self.splashes.iter_mut().enumerate() {
                if particle.update(&self.level) {
                    remove.push(i);
                }
            }

            for i in remove.into_iter().rev() {
                let particle = self.splashes.remove(i);
                despawned_particles.push(particle);
            }
        }
        despawned_particles
    }
}
