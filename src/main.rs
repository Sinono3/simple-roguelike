mod creatures;
mod commands;
mod features;

use crate::creatures::*;
use crate::features::*;

fn main() {
	let human_warrior = Creature {
		name: String::from("human_warrior"),
		health: 20,
		damage: 4,
		features: vec![]
	};
	let goblin = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2,
		features: vec![Feature::Aggression]
	};
	let mut state = GameState::new(human_warrior.clone());

	state.creatures.add(goblin.clone());
	state.creatures.add(goblin.clone());

	loop {
		state.round();
	}
}
