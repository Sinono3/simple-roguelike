use super::Component;
use super::super::game_state::{GameState, PLAYER_ID};

#[derive(Clone)]
pub struct AggressionComponent;

impl From<AggressionComponent> for Component {
	fn from(component: AggressionComponent) -> Self { Component::Aggression }
}

pub fn aggression(state: &mut GameState) {
	let ids: Vec<usize> = state.creatures.all::<AggressionComponent>()
									.iter().enumerate()
									.filter_map(|(id, a)| a.clone().map(|_| id)).collect();
	for id in ids {
		state.hit(id, PLAYER_ID);
	}
}
