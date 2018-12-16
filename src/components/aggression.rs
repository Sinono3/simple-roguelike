use super::Component;
use super::super::game_state::{GameState, PLAYER_ID};

#[derive(Clone)]
pub struct AggressionComponent;

impl From<AggressionComponent> for Component {
	fn from(component: AggressionComponent) -> Self { Component::Aggression }
}

pub fn aggression(state: &mut GameState) {
	let components = state.creatures.all::<AggressionComponent>().iter().enumerate();
	for (id, component) in components {
		if let Some(_) = component {
			state.hit(id, PLAYER_ID);
		}
	}
}
