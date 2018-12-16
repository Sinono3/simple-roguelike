use super::super::game_state::{GameState, PLAYER_ID};

#[derive(Clone)]
pub struct AggressionComponent;

pub fn aggression(state: &mut GameState) {
	let ids: Vec<usize> = state.creatures.all::<AggressionComponent>()
									.iter().enumerate()
									.filter_map(|(id, a)| a.clone().map(|_| id)).collect();
	for id in ids {
		state.hit(id, PLAYER_ID);
	}
}