//extern crate anymap;
extern crate crossterm;
extern crate multi_mut;

use crossterm::terminal::*;
use crossterm::style::{Color, style};

mod game_state;
mod commands;
mod components;
mod util;

use crate::components::{EntityType, EntityData};
use crate::components::creature::{AttackComponent, AggressionComponent};
use crate::game_state::GameState;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let human_warrior = EntityData::new("human_warrior", 20, EntityType::Creature)
		.with(AttackComponent { damage: 4 });

	let goblin = EntityData::new("goblin", 12, EntityType::Creature)
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
