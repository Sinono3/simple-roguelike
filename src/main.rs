use std::io;
use std::io::prelude::*;

type CreatureId = usize;

fn pause() {
    // Read a single byte and discard
    let _ = io::stdin().read(&mut [0u8]).unwrap();
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

#[derive(Clone)]
struct Creature {
	name: String,
	health: i32,
	damage: i32
}

/*impl Creature {
	fn think(&mut self, battle: &mut Battle) {
		battle.attack(Attack {
			inflictor: *creature,
			victim: 0
		});
	}
}*/

struct GameState {
	creatures: Vec<Creature>,
	player: CreatureId,
	enemies: Vec<CreatureId>
}

impl GameState {
	pub fn new(player: Creature) -> GameState{
		let mut state = GameState {
			creatures: Vec::new(),
			player: 0,
			enemies: Vec::new()
		};
		state.add_creature(player);
		state
	}
	pub fn add_creature(&mut self, creature: Creature) -> CreatureId {
		self.creatures.push(creature);
		self.creatures.len() - 1
	}
	// TODO: be able to add more than 1 creature at once, and return a slice of creature ids.
	pub fn get_creature(&self, i: CreatureId) -> &Creature {
		&self.creatures[i]
	}
	pub fn get_creature_mut(&mut self, i: CreatureId) -> &mut Creature {
		&mut self.creatures[i]
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
	pub fn attack(&mut self, attack: Attack) {
		self.attack_batch.push(attack);
	}
	pub fn round(&mut self, state: &mut GameState) {
		// creatures thinking.
		/*for creature in &self.involved {
			creature.think(&mut self, );
		}*/
		
		// attacks are being processed.
		for attack in self.attack_batch.drain(..) {
			let inflictor = state.get_creature(attack.inflictor).clone();
			let victim = state.get_creature_mut(attack.victim);
			
			victim.health -= inflictor.damage;
			println!("{} {} hit {} for {} damage!", inflictor.name, attack.inflictor, victim.name, inflictor.damage.to_string());
			pause();
		}
	}
}

fn main() {
	let human_warrior = Creature {
		name: String::from("human warrior"),
		health: 20,
		damage: 4
	};
	let goblin = Creature {
		name: String::from("goblin"),
		health: 12,
		damage: 2
	};
	let mut state = GameState::new(human_warrior.clone());
	let mut battle = Battle::new();
	
	battle.involve(state.add_creature(goblin.clone()));
	
	loop {
		battle.attack(Attack {
				inflictor: 1,
				victim: 0
		});
		battle.round(&mut state);
	}
}
