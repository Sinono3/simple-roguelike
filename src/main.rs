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
use crate::creatures::*;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let human_warrior = CreatureData::new("human_warrior", 20)
									 .with(Box::new(AttackComponent { damage: 4 }));
	let goblin = CreatureData::new("goblin", 12)
							  .with(Box::new(AttackComponent { damage: 2 }))
							  .with(Box::new(AggressionComponent));
	let goblin2 = CreatureData::new("goblin", 12)
							   .with(Box::new(AttackComponent { damage: 2 }))
						  	   .with(Box::new(AggressionComponent));

	let mut state = GameState::new(human_warrior);

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
				   .with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
				   .with(Color::DarkGreen));

	state.add_register(goblin);
	state.add_register(goblin2);

	while state.round() {
		//playing
	}
}
