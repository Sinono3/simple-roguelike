//extern crate anymap;
extern crate crossterm;
extern crate multi_mut;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crossterm::terminal::*;
use crossterm::style::{Color, style};

mod game_state;
mod commands;
mod components;
mod util;

use crate::components::{EntityType, EntityData};
use crate::components::creature::{NeutralComponent, AttackComponent, AggressiveComponent};
use crate::components::shared::{OwnerComponent};
use crate::components::unanimate::*;
use crate::game_state::GameState;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let mut state = GameState::new();

	// items
	let rusty_sword = EntityData::new("rusty_sword", 18, EntityType::Unanimate)
			.with(OwnedComponent { owner: 0, entity_type: EntityType::Creature })
			.with(WieldableComponent { damage: 2 })
			.with(SalableComponent { worth: 10 });

	let stick = EntityData::new("stick", 18, EntityType::Unanimate)
			.with(OwnedComponent { owner: 1, entity_type: EntityType::Creature })
			.with(WieldableComponent { damage: 2 });

	let blood_dagger = EntityData::new("blood_dagger", 90, EntityType::Unanimate)
			.with(OwnedComponent { owner: 3, entity_type: EntityType::Creature })
			.with(WieldableComponent { damage: 8 })
			.with(SalableComponent { worth: 470 });

	state.unanimate.add(rusty_sword);
	state.unanimate.add(stick);
	state.unanimate.add(blood_dagger);

	// creatures
	let human_warrior = EntityData::new("human_warrior", 20, EntityType::Creature)
			.with(AttackComponent {
				strength: 2,
				wielding: Some(0)
			})
			.with(OwnerComponent {
				contents: vec![0],
			});

	use std::fs::File;
	use std::io::prelude::*;

    let mut file = File::open("./goblin.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

	/*let goblin = EntityData::new("goblin", 12, EntityType::Creature)
			.with(AttackComponent { strength: 1, wielding: None })
			.with(AggressiveComponent);*/
	let goblin: EntityData = serde_json::from_str(&contents).unwrap();

	let merchant = EntityData::new("merchant", 38, EntityType::Creature)
			.with(AttackComponent { strength: 1, wielding: Some(2) })
			.with(NeutralComponent::new());

	state.creatures.add(human_warrior);
	state.creatures.add(goblin.clone());
	state.creatures.add(goblin.with(AttackComponent { strength: 1, wielding: Some(1) }));
	state.creatures.add(merchant);

	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("\n## You're the only human warrior left and must defeat all enemies!\n")
			.with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
			.with(Color::DarkGreen));

	while state.round() {
		//playing
	}
}
