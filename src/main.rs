extern crate anymap;
extern crate crossterm;

use crossterm::terminal::*;
use crossterm::style::{Color, style};

mod game_state;
mod creatures;
mod commands;
mod components;


use crate::components::{AttackComponent, AggressionComponent};
use crate::game_state::GameState;
use crate::creatures::CreatureData;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let human_warrior = CreatureData::new("human_warrior", 20)
		.with(AttackComponent { damage: 4 });

	let goblin = CreatureData::new("goblin", 12)
		.with(AttackComponent { damage: 2 })
		.with(AggressionComponent);

	let mut state = GameState::new(human_warrior);

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
				   .with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
				   .with(Color::DarkGreen));

	state.creatures.add(goblin.clone());
	state.creatures.add(goblin);

	while state.round() {
		//playing
	}
}
