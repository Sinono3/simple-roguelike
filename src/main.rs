mod game_state;
mod creatures;
mod commands;
mod features;

extern crate crossterm;
use crossterm::terminal::*;
use crossterm::style::{Color, style};

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

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
				   .with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
				   .with(Color::DarkGreen));

	state.add_register(goblin.clone());
	state.add_register(goblin.clone());

	while state.round() {
		//playing
	}
}
