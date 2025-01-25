use crate::entity::player::Player;

#[derive(Debug, Clone)]
pub struct Game {
	pub player: Player,
}

impl Game {
	pub fn new() -> Game {
		Self {
			player: Player::default(),
		}
	}
}
