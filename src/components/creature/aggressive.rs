use crate::game_state::{GameState, PLAYER_ID};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AggressiveComponent;

use crate::components::{Component, ComponentType};
impl Component for AggressiveComponent {
	fn purpose() -> ComponentType { ComponentType::Creature }
}

pub fn aggressive(state: &mut GameState) {
	let ids: Vec<usize> = state.creatures.all::<AggressiveComponent>()
									.iter().enumerate()
									.filter_map(|(id, a)| a.clone().map(|_| id)).collect();
	for id in ids {
		state.hit(id, PLAYER_ID);
	}
}
