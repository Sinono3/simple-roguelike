use std::collections::HashMap;
use crate::components::{EntityType, ComponentMap, Entity, EntityData,
		EntityAllocator, shared::*, creature::*, unanimate::*};
use crate::util::anymap::AnyMap;

const ANYMAP_ERROR: &str = "Game logic error: Trying to add a different purpose entity to map.";

pub struct EntityMap {
	alloc: EntityAllocator,
	purpose: EntityType,
	pub components: AnyMap,
	name_count: HashMap<String, i32>,
}

#[allow(dead_code)]
impl EntityMap {
	pub fn new(p: EntityType) -> EntityMap {
		let mut entity_map = EntityMap {
			alloc: EntityAllocator::new(),
			purpose: p,
			components: AnyMap::new(),
			name_count: HashMap::new(),
		};
		entity_map.components.insert::<ComponentMap<NameComponent>>(Vec::new());
		entity_map.components.insert::<ComponentMap<HealthComponent>>(Vec::new());
		entity_map.components.insert::<ComponentMap<OwnerComponent>>(Vec::new());

		match entity_map.purpose {
			EntityType::Creature => {
				entity_map.components.insert::<ComponentMap<AttackComponent>>(Vec::new());
				entity_map.components.insert::<ComponentMap<AggressionComponent>>(Vec::new());
			}
			EntityType::Unanimate => {
				// TODO.
				entity_map.components.insert::<ComponentMap<OwnedComponent>>(Vec::new());
				entity_map.components.insert::<ComponentMap<WieldableComponent>>(Vec::new());
				entity_map.components.insert::<ComponentMap<SalableComponent>>(Vec::new());
			}
		}
		entity_map
	}

	pub fn add(&mut self, mut entity_data: EntityData) -> Entity {
		// get id and decide if allocating or not.
		let id = if let Some(id) = self.alloc.allocate() {
			id
		} else {
			push_none(self)
		};

		// adding adding shared components first.
		let new_name = if let Some(mut name) = entity_data.remove::<NameComponent>() {
			if let Some(count) = self.name_count.get_mut(name.0.as_str()) {
				*count += 1;
				name.0.push_str(&count.to_string());
			} else {
				self.name_count.insert(name.0.clone(), 1);
			}
			Some(name)
		} else {
			panic!("Entity with id {} has no name.", id);
		};
		self.set::<NameComponent>(id, new_name);
		self.set::<HealthComponent>(id, entity_data.remove::<HealthComponent>());
		self.set::<OwnerComponent>(id, entity_data.remove::<OwnerComponent>());

		// adding specific components (this is just temporary, there will later be a cleaner way)
		match self.purpose {
			EntityType::Creature => {
				self.set::<AttackComponent>(id, entity_data.remove::<AttackComponent>());
				self.set::<AggressionComponent>(id, entity_data.remove::<AggressionComponent>());
			}
			EntityType::Unanimate => {
				self.set::<OwnedComponent>(id, entity_data.remove::<OwnedComponent>());
				self.set::<WieldableComponent>(id, entity_data.remove::<WieldableComponent>());
				self.set::<SalableComponent>(id, entity_data.remove::<SalableComponent>());
			}
		}

		id
	}
	pub fn remove(&mut self, id: Entity) -> Option<EntityData> {
		let mut data = EntityData::new_empty(EntityType::Creature)
			.with_option(self.remove_component::<NameComponent>(id))
			.with_option(self.remove_component::<HealthComponent>(id))
			.with_option(self.remove_component::<OwnerComponent>(id));

		match self.purpose {
			EntityType::Creature => {
				data.add_option(self.remove_component::<AttackComponent>(id));
				data.add_option(self.remove_component::<AggressionComponent>(id));
			}
			EntityType::Unanimate => {
				data.add_option(self.remove_component::<OwnedComponent>(id));
				data.add_option(self.remove_component::<WieldableComponent>(id));
				data.add_option(self.remove_component::<SalableComponent>(id));
			}
		}

		self.alloc.deallocate(id);
		Some(data)
	}

	pub fn set<T: 'static>(&mut self, id: Entity, content: Option<T>) -> bool where T: Clone {
		let vec = self.components.get_mut::<ComponentMap<T>>();
		if let Some(v) = vec {
			v[id] = content;
			true
		} else {
			false
		}
	}

	pub fn get<T: 'static>(&self, id: Entity) -> Option<&T> where T: Clone {
		let vec = self.components.get::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		vec.get(id).expect(format!("Game logic error: Entity {} doesn't exist.", id)
				.as_str())
				.as_ref()
	}
	pub fn get_mut<T: 'static>(&mut self, id: Entity) -> Option<&mut T> where T: Clone {
		let vec = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		vec.get_mut(id).expect(format!("Game logic error: Entity {} doesn't exist.", id)
				.as_str())
				.as_mut()
	}

	pub fn all<T: 'static>(&self) -> &ComponentMap<T> where T: Clone {
		let map = self.components.get::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		map.as_ref()
	}
	pub fn all_mut<T: 'static>(&mut self) -> &mut Vec<Option<T>> where T: Clone {
		let map = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		map.as_mut()
	}

	pub fn remove_component<T: 'static>(&mut self, id: Entity) -> Option<T> where T: Clone {
		let map = self.components.get_mut::<ComponentMap<T>>().expect(ANYMAP_ERROR);
		let entity = map.remove(id);
		map.insert(id, None);
		entity
	}

	pub fn find_by_name(&self, name: &str) -> Option<Entity> {
		let map = self.all::<NameComponent>();

		map.iter().position(|x|
			if let Some(c) = x {
				c.0.as_str() == name
				} else {
				false
			}
		)
	}
	pub fn existing(&self) -> Vec<Entity> {
		(0..self.alloc.len())
				.filter(|id| self.alloc.exists(*id))
				.collect()
	}
	// BE CAREFUL. VERY UNSTABLE. NEEDS GENERATIONAL INDEXES ASAP.
	pub fn exists(&self, id: Entity) -> bool {
		self.alloc.exists(id)
	}
	pub fn len(&self) -> usize {
		self.alloc.len()
	}
}
fn push_none(map: &mut EntityMap) -> Entity {
	map.all_mut::<NameComponent>().push(None);
	map.all_mut::<HealthComponent>().push(None);
	map.all_mut::<OwnerComponent>().push(None);

	match map.purpose {
		EntityType::Creature => {
			map.all_mut::<AttackComponent>().push(None);
			map.all_mut::<AggressionComponent>().push(None);
		}
		EntityType::Unanimate => {
			// TODO.
			map.all_mut::<OwnedComponent>().push(None);
			map.all_mut::<WieldableComponent>().push(None);
			map.all_mut::<SalableComponent>().push(None);
		}
	}
	map.len() - 1
}
