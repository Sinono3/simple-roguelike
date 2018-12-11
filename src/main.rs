use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

type CreatureId = usize;

fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [1u8]).unwrap();
    let _ = io::stdin().read(&mut [1u8]).unwrap();
}

#[derive(Clone)]
enum Feature {
	Aggression
}

#[derive(Clone)]
struct Creature {
	name: String,
	health: i32,
	damage: i32,
	features: Vec<Feature>
}

struct GameState {
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
				Feature::Aggression => {
					self.aggressive.push(id);
				}
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
		// creatures thinking V3 (sorta ECS with components as features)
		player_system(self);
		aggressive_system(self);
	}
}

#[derive(Copy, Clone)]
struct Attack {
	inflictor: CreatureId,
	victim: CreatureId
}

fn aggressive_system(state: &mut GameState) {
	for i in 0..state.aggressive.len() {
		let aggressive_id = state.aggressive[i];
		
		let (name, damage) = {
			let thinker = state.get_creature(aggressive_id);
			(thinker.name.clone(), thinker.damage)
		};
		
		let mut victim = state.get_creature_mut(0);
		victim.health -= damage;
		println!("{} hit {} for {} damage!", name, victim.name, damage.to_string());
		pause();
	}
}

enum Command {
	Attack(CreatureId),
	Examine(CreatureId)
}

impl Command {
	fn get(state: &GameState) -> Command {
		let stdin = io::stdin();
		let mut buffer = String::new();
		
		loop {
			stdin.read_line(&mut buffer).unwrap();
			
			let parts: Vec<&str> = buffer.trim().split(' ').collect();
			
			match parts[0] {
				"attack" => {
					if parts.len() > 1 {
						if let Some(target) = state.find_creature(parts[1]) {
							break Command::Attack(target);
						}
					}
					println!("Please write a correct target: 'attack goblin1'");
				}
				"examine" => {
					if parts.len() > 1 {
						if let Some(target) = state.find_creature(parts[1]) {
							break Command::Examine(target);
						}
					}
					println!("Please write a correct target: 'examine goblin1'");
				}
				_ => {
					println!("'{}' is not a correct command.", parts[0]);
				}
			}
			
			buffer.clear();
		}
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
				let damage = state.get_creature(0).damage;
			
				let mut victim = state.get_creature_mut(target);
				victim.health -= damage;
				println!("You hit {} for {} damage!", victim.name, damage.to_string());
				break;
			}
			Command::Examine(target) => {
				let mut creature = state.get_creature(target);
				println!("{} has {} hitpoints remaining and does {} damage.", creature.name, creature.health, creature.damage);
			}
		}
		println!("Enter another command:");
	}
}

fn main() {
	let human_warrior = Creature {
		name: String::from("human warrior"),
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
	
	state.add_creature(goblin.clone());
	state.add_creature(goblin.clone());
	
	loop {
		state.round();
	}
}
