#[macro_use]
use std::collections::HashMap;
use anymap::any::Any;
use anymap::AnyMap;
use crate::components::*;

const ANYMAP_ERROR: &str = "Game internal error: CreatureMap does not have the respective component vector.";

pub type CreatureId = usize;

struct CreatureAllocator {
	free: Vec<CreatureId>,
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
	fn is_free(&self, id: CreatureId) -> bool {
		self.free.contains(&id)
	}
}
pub struct CreatureData {
	contents: Box<AnyMap>,
}
impl CreatureData {
	pub fn new(name: &str, health: i32) -> CreatureData {
		let mut data = CreatureData {
			contents: Box::new(AnyMap::new())
		};
		data.contents.insert(Box::new(NameComponent(String::from(name))));
		data.contents.insert(Box::new(HealthComponent(health)));
		data
	}
	pub fn with<T: 'static>(mut self, component: Box<T>) -> Self {
		self.contents.insert(component);
		self
	}
	pub fn contains<T: 'static>(&self) -> bool {
		self.contents.contains::<Box<T>>()
	}
	fn remove<T: 'static>(&mut self) -> Box<Option<T>> {
		let content = self.contents.remove::<Box<T>>();
		if let Some(c) = content {
			Box::new(Some(*c))
		} else {
			Box::new(None)
		}
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
	len: usize,
	name_count: HashMap<String, i32>,
}

impl CreatureMap {
	pub fn new() -> CreatureMap {
		let mut creature_map = CreatureMap {
			alloc: CreatureAllocator::new(),
			components: AnyMap::new(),
			len: 0,
			name_count: HashMap::new(),
		};
		let name_components: Vec<Option<NameComponent>> = Vec::new();
		creature_map.components.insert(name_components);

		let health_components: Vec<Option<HealthComponent>> = Vec::new();
		creature_map.components.insert(health_components);

		let attack_components: Vec<Option<AttackComponent>> = Vec::new();
		creature_map.components.insert(attack_components);

		let aggression_components: Vec<Option<AggressionComponent>> = Vec::new();
		creature_map.components.insert(aggression_components);

		creature_map
	}
	fn set_none(&mut self, id: CreatureId) {
		assert!(self.set::<NameComponent>(id, None), ANYMAP_ERROR);
		assert!(self.set::<HealthComponent>(id, None), ANYMAP_ERROR);
		assert!(self.set::<AttackComponent>(id, None), ANYMAP_ERROR);
		assert!(self.set::<AggressionComponent>(id, None), ANYMAP_ERROR);
	}
	fn push_none(&mut self) -> CreatureId {
		self.all_mut::<NameComponent>().push(None);
		self.all_mut::<HealthComponent>().push(None);
		self.all_mut::<AttackComponent>().push(None);
		self.all_mut::<AggressionComponent>().push(None);
		self.len += 1;
		self.len - 1
	}
	pub fn add(&mut self, mut creature: CreatureData) -> CreatureId {
		// get id and decide if allocating or not.
		let id = if let Some(id) = self.alloc.allocate() {
			self.set_none(id);
			id
		} else {
			self.push_none()
		};

		// prevent same name.
		let new_name = if let Some(mut name) = *creature.remove::<NameComponent>() {
			if let Some(count) = self.name_count.get_mut(name.0.as_str()) {
				*count += 1;
				name.0.push_str(&count.to_string());
			} else {
				self.name_count.insert(name.0.clone(), 1);
			}
			Some(name)
		} else {
			panic!("Creature with id {} has no name.", id);
		};

		// add components
		self.set::<NameComponent>(id, new_name);
		self.set::<HealthComponent>(id, *creature.remove::<HealthComponent>());
		self.set::<AttackComponent>(id, *creature.remove::<AttackComponent>());
		self.set::<AggressionComponent>(id, *creature.remove::<AggressionComponent>());

		id
	}
	// TODO: multiple concurrent component borrows, like a tuple (Component, Component...)
	pub fn get<T: 'static>(&self, id: CreatureId) -> Option<&T> {
		let vec = self.components.get::<Vec<Option<T>>>().expect(ANYMAP_ERROR);
		vec.get(id).expect("Game logic error: Creature doesn't exist.").as_ref()
	}
	pub fn get_mut<T: 'static>(&mut self, id: CreatureId) -> Option<&mut T> {
		let vec = self.components.get_mut::<Vec<Option<T>>>().expect(ANYMAP_ERROR);
		vec.get_mut(id).expect("Game logic error: Creature doesn't exist.").as_mut()
	}
	pub fn set<T: 'static>(&mut self, id: CreatureId, content: Option<T>) -> bool {
		let vec = self.components.get_mut::<Vec<Option<T>>>();
		if let Some(v) = vec {
			v[id] = content;
			true
		} else {
			false
		}
	}
	pub fn all<T: 'static>(&self) -> &Vec<Option<T>> {
		let vec = self.components.get::<Vec<Option<T>>>().expect(ANYMAP_ERROR);
		vec.as_ref()
	}
	pub fn all_mut<T: 'static>(&mut self) -> &mut Vec<Option<T>> {
		let vec = self.components.get_mut::<Vec<Option<T>>>().expect(ANYMAP_ERROR);
		vec.as_mut()
	}
	pub fn remove(&mut self, id: CreatureId) {
		self.set_none(id);
		self.alloc.deallocate(id);
	}
	pub fn find_by_name(&self, name: &str) -> Option<CreatureId> {
		let vec = self.components.get::<Vec<Option<NameComponent>>>().expect(ANYMAP_ERROR);

		vec.iter().position(|x|
			if let Some(c) = x {
				c.0.as_str() == name
				} else {
				false
			}
		)
	}
	pub fn len(&self) -> usize {
		self.len
	}
	pub fn alive(&self) -> Vec<CreatureId> {
		(0..self.len)
				.filter(|id| !self.alloc.is_free(*id))
				.collect()
	}
	#[allow(dead_code)]
	pub fn alive_count(&self) -> usize {
		(0..self.len)
				.filter(|id| !self.alloc.is_free(*id))
				.count()
	}
	// BE CAREFUL. VERY UNSTABLE. NEEDS GENERATION_INDEXES ASAP.
	pub fn is_dead(&self, id: CreatureId) -> bool {
		self.alloc.free.contains(&id)
	}
	/* Removed until needed again.
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
	}*/
}
