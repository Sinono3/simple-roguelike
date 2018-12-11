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

impl Creature {
	
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
	Attack(String)
}

impl Command {
	fn get() -> Command {
		let stdin = io::stdin();
		let mut buffer = String::new();
		
		loop {
			stdin.read_line(&mut buffer).unwrap();
			
			if let Some(first_word) = buffer.find(' ') {
				match &buffer[..first_word] {
					"attack" => { 
						if first
						let target = String::from(&buffer[first_word+1..]);
						break Command::Attack(target);
					}
					_ => {
						println!("{}", &buffer[..first_word]);
						println!("Please write a possible action: 'attack goblin1'");
					}
				}
			} else {
				println!("Please write a possible action: 'attack goblin1'");
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
	
	let chosen = Command::get();
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
