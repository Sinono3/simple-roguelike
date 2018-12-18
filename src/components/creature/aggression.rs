use crate::game_state::{GameState, PLAYER_ID};

#[derive(Clone, Debug)]
pub struct AggressionComponent;

use crate::components::{Component, ComponentPurpose};
impl Component for AggressionComponent {
	fn purpose() -> ComponentPurpose { ComponentPurpose::Creature }
}

pub fn aggression(state: &mut GameState) {
	let ids: Vec<usize> = state.creatures.all::<AggressionComponent>()
									.iter().enumerate()
									.filter_map(|(id, a)| a.clone().map(|_| id)).collect();
	for id in ids {
		state.hit(id, PLAYER_ID);
	}
}
