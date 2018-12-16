#[macro_use]
use std::any::TypeId;
use std::collections::HashMap;
use anymap::any::Any;
use anymap::AnyMap;

use crate::components::*;

pub type CreatureId = usize;

struct CreatureAllocator {
	free: Vec<CreatureId>
}
impl CreatureAllocator {
	fn new() -> CreatureAllocator {
		CreatureAllocator {
			free: Vec::new()
		}
	}
	fn allocate(&mut self) -> Option<CreatureId> {
		self.free.pop()
	}
	fn deallocate(&mut self, id: CreatureId) {
		self.free.push(id);
	}
}
pub struct CreatureData {
	contents: Box<AnyMap>,
}
impl CreatureData {
	pub fn new(name: &str, health: i32) -> CreatureData {
		let data = CreatureData {
			contents: Box::new(AnyMap::new())
		};
		data.contents.insert(NameComponent(String::from(name)));
		data.contents.insert(HealthComponent(health));
		data
	}
	pub fn with(self, component: Box<Any>) -> Self {
		self.contents.insert(component);
		self
	}
	pub fn contains<T: 'static>(&self) -> bool {
		self.contents.contains::<T>()
	}
	pub fn get<T: 'static>(&self) -> Option<&T> {
		self.contents.get::<T>()
	}
	fn remove<T: 'static>(&self) -> Option<T> {
		self.contents.remove::<T>()
	}
	/*pub fn clone(&self) -> CreatureData {
		let deref = *self.contents.deref();
		CreatureData {
			contents: Box::new(deref.clone())
		}
	}*/
}

pub struct CreatureMap {
	alloc: CreatureAllocator,
	components: AnyMap,
	name_count: HashMap<String, i32>,
}

impl CreatureMap {
	pub fn new() -> CreatureMap {
		let creature_map = CreatureMap {
			alloc: CreatureAllocator::new(),
			components: AnyMap::new(),
			name_count: HashMap::new(),
		};
		let name_components: Vec<Option<NameComponent>> = Vec::new();
		creature_map.components.insert(name_components);

		let name_components: Vec<Option<HealthComponent>> = Vec::new();
		creature_map.components.insert(name_components);

		let name_components: Vec<Option<AttackComponent>> = Vec::new();
		creature_map.components.insert(name_components);

		let name_components: Vec<Option<AggressionComponent>> = Vec::new();
		creature_map.components.insert(name_components);

		creature_map
	}
	pub fn add(&mut self, mut creature: CreatureData) -> CreatureId {
		// get id and decide if allocating or not.
		let id = if let Some(id) = self.alloc.allocate() {
			self.name_components[id] = None;
			self.health_components[id] = None;
			self.attack_components[id] = None;
			self.aggression_components[id] = None;
			id
		} else {
			self.name_components.push(None);
			self.health_components.push(None);
			self.attack_components.push(None);
			self.aggression_components.push(None);
			self.len() - 1
		};

		// prevent same name.
		let new_name = if let Some(name) = creature.remove::<NameComponent>() {
			if let Some(count) = self.name_count.get_mut(name.0.as_str()) {
				*count += 1;
				name.0.push_str(&count.to_string());
			} else {
				self.name_count.insert(name.0.clone(), 1);
			}
			Some(name)
		} else {
			panic!("Creature with id {} has no name.", id);
			None
		};

		// add components
		self.name_components[id] = new_name;
		self.health_components[id] = creature.remove::<HealthComponent>();
		self.attack_components[id] = creature.remove::<AttackComponent>();
		self.aggression_components[id] = creature.remove::<AggressionComponent>();

		id
	}
	// TODO: be able to add more than 1 creature at once, and return a slice of creature ids.
	pub fn get<T: 'static>(&self, id: CreatureId) -> Option<T> {
		// TEMPORAL SOLUTION!!!!!
		match TypeId::of::<T>() {
			NAME_COMPONENT_ID => self.name_components.get(id),
			HEALTH_COMPONENT_ID => self.health_components.get(id),
			ATTACK_COMPONENT_ID => self.attack_components.get(id),
			AGGRESSION_COMPONENT_ID => self.aggression_components.get(id)
		}
	}
	pub fn get_mut(&mut self, id: CreatureId) -> Option<&mut T> {
		self.vec[id].as_mut()
	}
	pub fn remove(&mut self, id: CreatureId) {
		let removed = self.vec.remove(id).expect("Game logic error: trying to remove unexisting creature.");
		self.vec.insert(id, None);
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
		self.name_components.len()
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
