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
use crate::weapons::*;
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

	create_weapons(&mut state);
	while state.round() {
		//playing
	}
}
fn create_weapons(_state : &mut GameState){
	let big_sword = Weapon {
		name : String::from("big_sword"),
		demage : 6
	};
	let stick = Weapon {
		name : String::from("stick"),
		demage : 2
	};
	let snife = Weapon {
		name : String::from("snife"),
		demage : 4
	};

	_state._weapon_manager.add_weapon(big_sword.clone());
	_state._weapon_manager.add_weapon(stick.clone());
	_state._weapon_manager.add_weapon(snife.clone());
}
