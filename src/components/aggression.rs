use super::Component;
use super::super::game_state::{GameState, PLAYER_ID};

#[derive(Clone)]
pub struct AggressionComponent;

impl From<AggressionComponent> for Component {
	fn from(component: AggressionComponent) -> Self { Component::Aggression }
}

pub fn aggression_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		state.hit(state.aggressive[i], PLAYER_ID);
	}
}
