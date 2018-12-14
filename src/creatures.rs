use std::collections::HashMap;

pub type CreatureId = usize;

use crate::features::Feature;

#[derive(Clone)]
pub struct Creature {
	pub name: String,
	pub health: i32,
	pub damage: i32,
	pub features: Vec<Feature>
}

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
	pub fn len(&self) -> usize {
		self.vec.len()
	}
	/*#[allow(dead_code)]
	pub fn iter(&mut self) -> std::slice::IterMut<Option<Creature>> {
		self.vec.iter_mut()
	}*/
	pub fn alive(&self) -> Vec<CreatureId> {
		self.vec.iter()
				.enumerate()
				.filter(|(_, x)| x.is_some())
				.map(|(id, _)| id)
				.collect()
	}
	#[allow(dead_code)]
	pub fn alive_count(&self) -> usize {
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
