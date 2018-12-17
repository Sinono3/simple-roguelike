use std::collections::HashMap;
use anymap::AnyMap;
use super::{CreatureId, creature_allocator::CreatureAllocator, creature_data::CreatureData};
use crate::components::{*, map::ComponentMap};

const ANYMAP_ERROR: &str = "Game logic error: CreatureMap does not contain the respective component vector.";

pub struct CreatureMap {
	alloc: CreatureAllocator,
	pub components: AnyMap,
	name_count: HashMap<String, i32>,
}

#[allow(dead_code)]
impl CreatureMap {
	pub fn new() -> CreatureMap {
		let mut creature_map = CreatureMap {
			alloc: CreatureAllocator::new(),
			components: AnyMap::new(),
			name_count: HashMap::new(),
		};

		creature_map.components.insert::<ComponentMap<NameComponent>>(Vec::new());
		creature_map.components.insert::<ComponentMap<HealthComponent>>(Vec::new());
		creature_map.components.insert::<ComponentMap<AttackComponent>>(Vec::new());
		creature_map.components.insert::<ComponentMap<AggressionComponent>>(Vec::new());
		creature_map
	}
	pub fn add(&mut self, mut creature_data: CreatureData) -> CreatureId {
		// get id and decide if allocating or not.
		let id = if let Some(id) = self.alloc.allocate() {
			set_none(self, id);
			id
		} else {
			push_none(self)
		};

		// prevent same name.
		let new_name = if let Some(mut name) = creature_data.remove::<NameComponent>() {
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
		self.set::<HealthComponent>(id, creature_data.remove::<HealthComponent>());
		self.set::<AttackComponent>(id, creature_data.remove::<AttackComponent>());
		self.set::<AggressionComponent>(id, creature_data.remove::<AggressionComponent>());

		id
	}
	// TODO: multiple concurrent component borrows, like a tuple (Component, Component...)
	pub fn get<T: 'static>(&self, id: CreatureId) -> Option<&T> where T: Clone {
		let vec = self.components.get::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		vec.get(id).expect(format!("Game logic error: Creature {} doesn't exist.", id)
				.as_str())
				.as_ref()
	}
	pub fn get_mut<T: 'static>(&mut self, id: CreatureId) -> Option<&mut T> where T: Clone {
		let vec = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		vec.get_mut(id).expect(format!("Game logic error: Creature {} doesn't exist.", id)
				.as_str())
				.as_mut()
	}
	/*pub fn get_two_mut<T: 'static, T2: 'static>(&mut self, id: CreatureId)
			-> (Option<&mut T>, Option<&mut T2>) where T: Clone, T2: Clone {
		(self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR)
				.get_mut(id).expect(format!("Game logic error: Creature {} doesn't exist.", id)
				.as_str())
				.as_mut(),
		self.components.get_mut::<ComponentMap<T2>>().expect(ANYMAP_ERROR)
				.get_mut(id).expect(format!("Game logic error: Creature {} doesn't exist.", id)
				.as_str())
				.as_mut())
	}*/
	pub fn set<T: 'static>(&mut self, id: CreatureId, content: Option<T>) -> bool where T: Clone {
		let vec = self.components.get_mut::<ComponentMap<T>>();
		if let Some(v) = vec {
			v[id] = content;
			true
		} else {
			false
		}
	}
	pub fn all<T: 'static>(&self) -> &ComponentMap<T> where T: Clone {
		let map = self.components.get::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		map.as_ref()
	}
	pub fn all_mut<T: 'static>(&mut self) -> &mut Vec<Option<T>> where T: Clone {
		let map = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		map.as_mut()
	}
	pub fn remove(&mut self, id: CreatureId) -> Option<CreatureData> {
		let data = CreatureData::new_empty()
			.with_option(self.remove_component::<NameComponent>(id))
			.with_option(self.remove_component::<HealthComponent>(id))
			.with_option(self.remove_component::<AttackComponent>(id))
			.with_option(self.remove_component::<AggressionComponent>(id));

		self.alloc.deallocate(id);
		Some(data)
	}
	pub fn remove_component<T: 'static>(&mut self, id: CreatureId) -> Option<T> where T: Clone {
		let map = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		let creature = map.remove(id);
		map.insert(id, None);
		creature
	}
	pub fn find_by_name(&self, name: &str) -> Option<CreatureId> {
		let map = self.all::<NameComponent>();

		map.iter().position(|x|
			if let Some(c) = x {
				c.0.as_str() == name
				} else {
				false
			}
		)
	}
	pub fn alive(&self) -> Vec<CreatureId> {
		(0..self.alloc.len())
				.filter(|id| self.alloc.exists(*id))
				.collect()
	}
	// BE CAREFUL. VERY UNSTABLE. NEEDS GENERATIONAL INDEXES ASAP.
	pub fn exists(&self, id: CreatureId) -> bool {
		self.alloc.exists(id)
	}
	pub fn len(&self) -> usize {
		self.alloc.len()
	}
}
fn set_none(map: &mut CreatureMap, id: CreatureId) {
	assert!(map.set::<NameComponent>(id, None), ANYMAP_ERROR);
	assert!(map.set::<HealthComponent>(id, None), ANYMAP_ERROR);
	assert!(map.set::<AttackComponent>(id, None), ANYMAP_ERROR);
	assert!(map.set::<AggressionComponent>(id, None), ANYMAP_ERROR);
}
fn push_none(map: &mut CreatureMap) -> CreatureId {
	map.all_mut::<NameComponent>().push(None);
	map.all_mut::<HealthComponent>().push(None);
	map.all_mut::<AttackComponent>().push(None);
	map.all_mut::<AggressionComponent>().push(None);
	map.len() - 1
}
