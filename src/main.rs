mod game_state;
mod creatures;
mod commands;
mod features;
mod weapons;

extern crate crossterm;
use crossterm::terminal::*;

use crate::features::Feature;
use crate::game_state::GameState;
use crate::creatures::*;

fn main() {

	let terminal = terminal();
	terminal.clear(ClearType::All);

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

	println!("##########################################");
	println!("######### Simple Rusty Roguelike #########");
	println!("##########################################");

	println!("\n## You're the only human warrior left and must defeat all enemies!\n");
	println!("Type 'help' to see the available commands.");

	state.add_register(goblin.clone());
	state.add_register(goblin.clone());

	while state.round() {
		//playing
	}
}
