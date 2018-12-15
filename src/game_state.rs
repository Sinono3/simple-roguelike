use crate::creatures::*;
use crate::features::{Feature, aggressive_system, player_system};

pub const PLAYER_ID: CreatureId = 0;

use crate::weapons::*;
pub struct GameState {
	pub creatures: CreatureMap,
	pub aggressive: Vec<CreatureId>,
	pub _weapon_manager : WeaponManager
}

impl GameState {
	pub fn new(player: Creature) -> GameState {
		let mut state = GameState {
			creatures: CreatureMap::new(),
			aggressive: Vec::new(),
			_weapon_manager: WeaponManager::new()
		};
		state.creatures.add(player);
		state
	}
	pub fn add_register(&mut self, creature: Creature) -> CreatureId {
		let id = self.creatures.len();

		for feature in &creature.features {
			match feature {
				Feature::Aggression => self.aggressive.push(id)
			}
		}

		self.creatures.add(creature)
	}
	#[allow(dead_code)]
	pub fn remove_feature(&mut self, id: CreatureId, feature: Feature) {
		let creature = self.creatures.get_mut(id)
									 .expect("Game logic error: can't remove if feature if creature doesn't exist.");
		if let Some(feature_index) = creature.features.iter().position(|x| *x == feature) {
			creature.features.remove(feature_index);
		}
	}
	pub fn round(&mut self) -> bool {
		// systems.
		player_system(self);
		aggressive_system(self);

		true // TODO: player_system can return this, if not then the game will close because of the player's will
	}
	// Hits a creature with the inflictor's name and damage.
	pub fn hit(&mut self, inflictor_id: CreatureId, target_id: CreatureId) {

		assert!(inflictor_id != target_id, "Game logic error: a creature can't attack itself.");

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
		let target_str = if target_id == PLAYER_ID {
				"you".to_owned()
			} else {
				target_name
			};
		let inflictor_str = if inflictor_id == PLAYER_ID {
				"++ You hit".to_owned()
			} else {
				format!("{} hit", name)
			};
		println!("{} {} for {} damage.", inflictor_str, target_str, damage.to_string());

		if target_health > 0 {
			if target_id != PLAYER_ID {
				println!("> {} now has {} hitpoints remaining.", target_str, target_health.to_string());
			}
		} else {
			self.die(target_id);
		}
	}
	pub fn die(&mut self, dead_id: CreatureId) {
		let creature = self.creatures.remove(dead_id);

		let error_str = "Game internal error: creature with feature is not on its respective list.";
		for feature in creature.features {
			match feature {
				Feature::Aggression => self.aggressive.remove(self.aggressive.iter()
																			 .position(|x| *x == dead_id)
																			 .expect(error_str))
			};
		}

		let target_str = if dead_id == PLAYER_ID {
						 	 "You died!".to_owned()
						 } else {
						 	 format!("{} has died!", creature.name)
						 };

		println!("{}", target_str);
	}
}
