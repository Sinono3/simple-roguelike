mod creatures;
mod commands;
mod features;

use crate::creatures::*;
use crate::features::*;

fn main() {
	let human_warrior = Creature {
		name: String::from("human warrior"),
		health: 20,
		damage: 4,
		features: vec![Feature::Playable]
	};
	let goblin = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2,
		features: vec![Feature::Aggression]
	};
	let mut state = GameState::new(human_warrior.clone());
	
	state.add_creature(goblin.clone());
	state.add_creature(goblin.clone());
	
	loop {
		state.round();
	}
}
