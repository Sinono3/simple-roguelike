use crate::game_state::{GameState, PLAYER_ID};
use crate::creatures::Creature;
use crate::commands::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Feature {
	Aggression
}

pub fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		state.hit(state.aggressive[i], PLAYER_ID);
	}
}

pub fn player_system(state: &mut GameState) {
	// Can unwrap here because the player should exist.
	// If not then why should the game even be running.
	let player_health = state.creatures.get(PLAYER_ID)
									   .expect("Game logic error: the player is dead and the game is still running.")
									   .health;

	// Player control consists of three phases:
	// 1- Show the enviroment and conditions:
	println!("== You have {} hitpoints remaining.", player_health);

	let mut creature_string = String::new();

	let mut count = 0usize;
	// Can unwrap because alive() ASSURES that the returned creatures are alive.
	for creature in state.creatures.alive().iter()
										   .filter(|id| **id != PLAYER_ID)
										   .map(|id| state.creatures.get(*id)
										   .expect("Game internal error: alive() function returned a None.")) {
		creature_string.push_str(
			format!("{}; ", creature.name).as_str()
		);
		count += 1;
	}

	if count == 0 {
		println!("=============== You WIN! ==============");
	}
	else {
			println!("== There are {} enemies remaining: {}", count.to_string(), creature_string);
	}

	// 2- Ask for player input
	println!("Enter a command:");
	loop {
		let chosen = Command::get(state);

		// 3- Process the input.
		match chosen {
			Command::Attack(target) => {
				break state.hit(PLAYER_ID, target);
			}
			Command::Examine(target) => {
				let creature = state.creatures.get(target)
											  .expect("Game logic error: if the player is choosing this creature then it must exist.");
				println!("{} has {} hitpoints remaining and does {} damage.", creature.name, creature.health, creature.damage);
				pause();
			}
			Command::Status => {
				println!("== You have {} hitpoints remaining.", player_health);
				println!("== There are {} enemies remaining: {}", count.to_string(), creature_string)
			}
			Command::Help => {
				println!("The available commands are:
attack: Hit enemies. Usage: 'attack enemy_name'
examine: Shows the status of a creature. Usage: 'examine enemy_name'
status: Show your character's status and remaining enemies."
				);
			}
			Command::Debug(DebugCommand::Remove(target)) => {
				let creature: Creature = state.creatures.remove(target);
				println!("Creature '{}' with the id {} has been removed from the game.", creature.name, target);
			}
		}
		println!("Enter another command:");
	}
}
