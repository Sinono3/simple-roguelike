use std::io;
use std::io::prelude::*;

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
	player: CreatureId,
	aggressive: Vec<CreatureId>
}

impl GameState {
	pub fn new(player: Creature) -> GameState{
		let mut state = GameState {
			creatures: Vec::new(),
			player: 0,
			aggressive: Vec::new()
		};
		state.add_creature(player);
		state
	}
	pub fn add_creature(&mut self, creature: Creature) -> CreatureId {
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
}

#[derive(Copy, Clone)]
struct Attack {
	inflictor: CreatureId,
	victim: CreatureId
}

struct Battle {
	involved: Vec<CreatureId>,
	attack_batch: Vec<Attack>
}

impl Battle {
	pub fn new() -> Battle {
		Battle {
			involved: Vec::new(),
			attack_batch: Vec::new()
		}
	}
	pub fn involve(&mut self, id: CreatureId) {
		self.involved.push(id);
		// TODO: add a message announcing this.
	}
	// TODO: be able to involve more than one creature at once.
	pub fn round(&mut self, state: &mut GameState) {
		// creatures thinking V3 (sorta ECS with components as features)
		for i in 0..state.aggressive.len() {
			let aggressive_id = state.aggressive[i];
			
			if self.involved.contains(&aggressive_id) {
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
	let _goblin1 = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2,
		features: vec![]
	};
	let mut state = GameState::new(human_warrior.clone());
	let mut battle = Battle::new();
	
	battle.involve(0);
	battle.involve(state.add_creature(goblin.clone()));
	
	loop {
		battle.round(&mut state);
	}
}
