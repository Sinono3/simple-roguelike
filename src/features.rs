use std::collections::HashMap;

use crate::creatures::{Creature, CreatureId};
use crate::commands::*;

pub struct GameState {
	creatures: Vec<Creature>,
	names: HashMap<String, i32>,
	player: CreatureId,
	aggressive: Vec<CreatureId>
}

impl GameState {
	pub fn new(player: Creature) -> GameState {
		let mut state = GameState {
			creatures: Vec::new(),
			names: HashMap::new(),
			player: 0,
			aggressive: Vec::new()
		};
		state.add_creature(player);
		state
	}
	pub fn add_creature(&mut self, mut creature: Creature) -> CreatureId {
		// prevent same name.
		if let Some(count) = self.names.get_mut(&creature.name) {
			*count += 1;
			creature.name.push_str(&count.to_string());
		} else {
			self.names.insert(creature.name.clone(), 1);
		}
		
		// put in respective featured list.
		let id = self.creatures.len();
		for feature in &creature.features {
			match feature {
				Feature::Aggression => self.aggressive.push(id),
				Feature::Playable => ()
			}
		}
		self.creatures.push(creature);
		id
	}
	// TODO: be able to add more than 1 creature at once, and return a slice of creature ids.
	pub fn get_creature(&self, id: CreatureId) -> &Creature {
		&self.creatures[id]
	}
	pub fn get_creature_mut(&mut self, id: CreatureId) -> &mut Creature {
		&mut self.creatures[id]
	}
	pub fn find_creature(&self, name: &str) -> Option<CreatureId> {
		self.creatures.iter().position(|x| x.name.as_str() == name)
	}
	pub fn round(&mut self) {
		// everything thinking V3 (sorta ECS with components as features)
		player_system(self);
		aggressive_system(self);
	}
	// Hits a creature with the inflictor's name and damage.
	pub fn hit(&mut self, inflictor_id: CreatureId, target_id: CreatureId) {
		// get name and damage from inflictor
		let (name, damage) = {
			let inflictor = self.get_creature(inflictor_id);
			(inflictor.name.clone(), inflictor.damage)
		};
		// get name and apply damage to target
		let target_name = {
			let target = self.get_creature_mut(target_id);
			target.health -= damage;
			target.name.clone()
		};
		// english stuff
		let target_str = if target_id == self.player {
				"you".to_owned()
			} else {
				target_name
			};
		let inflictor_str = if inflictor_id == self.player {
				"You hit".to_owned()
			} else {
				format!("{} hit", name)
			};
		println!("{} {} for {} damage.", inflictor_str, target_str, damage.to_string());
	}
}

#[derive(Clone)]
pub enum Feature {
	Aggression,
	Playable
}

fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		state.hit(state.aggressive[i], 0); // THE 0 IS FOR TESTING PURPOSES
	}
}

fn player_system(state: &mut GameState) {
	// Player control consists of three phases:
	let player = state.get_creature_mut(state.player);
	
	// Show the enviroment and conditions:
	println!("You have {} hitpoints left.", player.health);
	
	let mut creature_string = String::new();
	
	for (_id, creature) in state.creatures.iter()
										  .enumerate()
										  .filter(|(id, _x)| *id != state.player) {
		creature_string.push_str(
			format!("{}; ", creature.name).as_str()
			);
	}
	println!("There are {} enemies: {}", (state.creatures.len() - 1).to_string(), creature_string);
	
	// Ask for player input
	println!("Enter a command:");
	loop {
		let chosen = Command::get(state);
		
		match chosen {
			Command::Attack(target) => {
				state.hit(state.player, target);
				break;
			}
			Command::Examine(target) => {
				let creature = state.get_creature(target);
				println!("{} has {} hitpoints remaining and does {} damage.", creature.name, creature.health, creature.damage);
			}
		}
		println!("Enter another command:");
	}
}