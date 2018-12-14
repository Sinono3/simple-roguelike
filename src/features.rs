use std::collections::HashMap;
use crate::creatures::{Creature, CreatureId};
use crate::commands::*;

const PLAYER_INDEX: CreatureId = 0;

pub struct CreatureMap {
	vec: Vec<Option<Creature>>,
	names: HashMap<String, i32>
}

impl CreatureMap {
	pub fn new() -> CreatureMap {
		CreatureMap {
			vec: Vec::new(),
			names: HashMap::new()
		}
	}
	pub fn add(&mut self, mut creature: Creature) -> CreatureId {
		// prevent same name.
		if let Some(count) = self.names.get_mut(&creature.name) {
			*count += 1;
			creature.name.push_str(&count.to_string());
		} else {
			self.names.insert(creature.name.clone(), 1);
		}

		let id = if let Some(x) = self.vec.iter().position(|x| x.is_none()) {
			self.vec[x] = Some(creature);
			x
		} else {
			self.vec.push(Some(creature));
			self.vec.len() - 1
		};
		/* TODO: Implement this on the GameState instead of here.
		// put in respective featured list.
		for feature in &creature.features {
			match feature {
				Feature::Aggression => self.vec.push(id),
				Feature::Playable => ()
			}
		}
		*/
		id
	}
	// TODO: be able to add more than 1 creature at once, and return a slice of creature ids.
	pub fn get(&self, id: CreatureId) -> Option<&Creature> {
		self.vec[id].as_ref()
	}
	pub fn get_mut(&mut self, id: CreatureId) -> Option<&mut Creature> {
		self.vec[id].as_mut()
	}
	pub fn remove(&mut self, id: CreatureId) -> Creature {
		let removed = self.vec.remove(id).expect("Game logic error: trying to remove unexisting creature.");
		self.vec.insert(id, None);
		removed
	}
	pub fn find(&self, name: &str) -> Option<CreatureId> {
		self.vec.iter().position(|x| {
			if let Some(creature) = x {
				creature.name.as_str() == name
				} else {
				false
			}
		})
	}
	#[allow(dead_code)]
	pub fn iter(&mut self) -> std::slice::IterMut<Option<Creature>> {
		self.vec.iter_mut()
	}
	pub fn alive(&self) -> Vec<CreatureId> {
		// Can unwrap because it is confirmed that the creature exists.
		self.vec.iter()
				.enumerate()
				.filter(|(_, x)| x.is_some())
				.map(|(id, _)| id)
				.collect()
	}
	#[allow(dead_code)]
	pub fn count(&self) -> usize {
		self.vec.iter()
				.filter(|x| x.is_some())
				.count()
	}
	#[allow(dead_code)]
	pub fn debug_slots(&self) {
		println!("Printing all {} creatures.", self.vec.len());
		for (id, option) in self.vec.iter().enumerate() {
			if let Some(c) = option {
				println!("Creature {}: {}", id, c.name);
			} else {
				println!("Creature {}: None", id);
			}
		}
	}
}

pub struct GameState {
	pub creatures: CreatureMap,
	aggressive: Vec<CreatureId>
}

impl GameState {
	pub fn new(player: Creature) -> GameState {
		let mut state = GameState {
			creatures: CreatureMap::new(),
			aggressive: Vec::new()
		};
		state.creatures.add(player);
		state
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
			// Can use unwrap because the target the inflictor is hitting must exist
			let inflictor = self.creatures.get(inflictor_id)
										  .expect("Game logic error: the inflictor must exist, in order to call this function.");
			(inflictor.name.clone(), inflictor.damage)
		};
		// get name and apply damage to target
		let (target_name, target_health) = {
			// Can unwrap because the target must be alive.
			let target = self.creatures.get_mut(target_id)
									   .expect("Game logic error: the target must exist, in order to be hit.");
			target.health -= damage;
			(target.name.clone(), target.health)
		};
		// english stuff
		let target_str = if target_id == PLAYER_INDEX {
				"you".to_owned()
			} else {
				target_name
			};
		let inflictor_str = if inflictor_id == PLAYER_INDEX {
				"You hit".to_owned()
			} else {
				format!("{} hit", name)
			};
		println!("{} {} for {} damage.", inflictor_str, target_str, damage.to_string());

		if target_health > 0 && target_id != PLAYER_INDEX {
			println!("{} now has {} hitpoints remaining.", target_str, target_health.to_string());
		} else {
			self.die(target_id);
		}
		pause();
	}
	pub fn die(&mut self, dead_id: CreatureId) {
		let creature = self.creatures.remove(dead_id);

		let target_str = if dead_id == PLAYER_INDEX {
						 	 "You died!".to_owned()
						 } else {
						 	 format!("{} has died!", creature.name)
						 };

		println!("{}", target_str);
	}
}

#[derive(Clone)]
pub enum Feature {
	Aggression
}

fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		state.hit(state.aggressive[i], PLAYER_INDEX); // THE 0 IS FOR TESTING PURPOSES
	}
}

fn player_system(state: &mut GameState) {
	// Can unwrap here because the player should exist.
	// If not then why should the game even be running.
	let player = state.creatures.get(PLAYER_INDEX)
								.expect("Game logic error: the player is dead and the game is still running.");

	// Player control consists of three phases:
	// 1- Show the enviroment and conditions:
	println!("== You have {} hitpoints left.", player.health);

	let mut creature_string = String::new();

	let mut count = 0usize;
	// Can unwrap because alive() ASSURES that the returned creatures are alive.
	for creature in state.creatures.alive().iter()
										   .filter(|id| **id != PLAYER_INDEX)
										   .map(|id| state.creatures.get(*id)
										   .expect("Game internal error: alive() function returned a None.")) {
		creature_string.push_str(
			format!("{}; ", creature.name).as_str()
		);
		count += 1;
	}
	println!("== There are {} enemies: {}", count.to_string(), creature_string);

	// 2- Ask for player input
	println!("Enter a command:");
	loop {
		let chosen = Command::get(state);

		// 3- Process the input.
		match chosen {
			Command::Attack(target) => {
				state.hit(PLAYER_INDEX, target);
				break;
			}
			Command::Examine(target) => {
				let creature = state.creatures.get(target)
											  .expect("Game logic error: if the player is choosing this creature then it must exist.");
				println!("{} has {} hitpoints remaining and does {} damage.", creature.name, creature.health, creature.damage);
				pause();
			}
			Command::Debug(DebugCommand::Remove(target)) => {
				let creature = state.creatures.remove(target);
				println!("Creature '{}' with the id {} has been removed from the game.", creature.name, target);
				pause();
				break;
			}
		}
		println!("Enter another command:");
	}
}
