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
		// queues which will be executed at the end
		attack_queue(self);
	}
}

#[derive(Clone)]
pub enum Feature {
	Aggression,
	Playable
}

fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		let creature = state.get_creature_mut(state.aggressive[i]);
		creature.current_victims.push(0);
	}
}

fn player_system(state: &mut GameState) {
	// Player control consists of three phases:
	let player = state.get_creature_mut(state.player);
	
	// Show the enviroment and conditions:
	println!("You have {} hitpoints left.", player.health);
	
	let mut creature_string = String::new();
	
	for id in 0..state.creatures.len() {
		if id == state.player {
			continue;
		}
		let creature = state.get_creature(id);
		creature_string.push_str(
			format!("{}; ", creature.name).as_str()
			);
	}
	println!("There are {} enemies: {}", (state.creatures.len() - 1).to_string(), creature_string);
	
		println!("Enter a command:");
	loop {
		let chosen = Command::get(state);
		
		match chosen {
			Command::Attack(target) => {
				state.get_creature_mut(0).current_victims.push(target);
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
fn attack_queue(state: &mut GameState) {
	for id in 0..state.creatures.len() {
		let (name, damage, victims) = {
			let inflictor = state.get_creature_mut(id);
			
			if inflictor.current_victims.len() < 1 {
				continue;
			}
			(inflictor.name.clone(), inflictor.damage, inflictor.current_victims.clone())
		};
		
		let player = state.player;
		for victim_id in victims {
			let mut victim = state.get_creature_mut(victim_id);
			victim.health -= damage;
			
			if id != player {
				println!("{} hit {} for {} damage!", name, victim.name, damage.to_string());
			} else {
				println!("You hit {} for {} damage!", victim.name, damage.to_string());
			}
			
			pause();
		}
		state.get_creature_mut(id).current_victims.clear();
	}
}