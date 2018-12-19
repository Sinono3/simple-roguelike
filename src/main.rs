//extern crate anymap;
extern crate crossterm;
extern crate multi_mut;
extern crate anymap;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crate::components::creature::AttackComponent;
use crossterm::terminal::*;
use crossterm::style::{Color, style};

mod game_state;
mod commands;
mod components;

use crate::components::{EntityType, EntityData};
use crate::game_state::GameState;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let mut state = GameState::new();

	// items
	state.unanimate.add(EntityData::load_from_json("unanimate/rusty_sword.json"));
	state.unanimate.add(EntityData::load_from_json("unanimate/stick.json"));
	state.unanimate.add(EntityData::load_from_json("unanimate/blood_dagger.json"));

	// creatures
	let human_warrior = EntityData::load_from_json("creatures/warrior.json");
	let goblin = EntityData::load_from_json("creatures/goblin.json");
	let merchant = EntityData::load_from_json("creatures/merchant.json");

	state.creatures.add(human_warrior);
	state.creatures.add(goblin.clone());
	state.creatures.add(goblin);
	state.creatures.add(merchant);

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
			.with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
			.with(Color::DarkGreen));

	state.owns(0, EntityType::Creature, 0);
	state.owns(2, EntityType::Creature, 1);
	state.owns(3, EntityType::Creature, 2);

	state.wields(0, 0);
	state.wields(2, 1);
	state.wields(3, 2);

	while state.round() {
		//playing
	}
}
