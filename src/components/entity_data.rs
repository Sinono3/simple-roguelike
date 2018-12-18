use crate::util::anymap::{Map, any::CloneAny};
use super::{Component, ComponentType, EntityType};
use super::shared::{NameComponent, HealthComponent};

type CloneMap = Map<CloneAny>;

#[derive(Clone)]
pub struct EntityData {
	contents: CloneMap,
	purpose: EntityType,
}
#[allow(dead_code)]
impl EntityData {
	pub fn new_empty(p: EntityType) -> EntityData {
		EntityData {
			contents: Map::<CloneAny>::new(),
			purpose: p
		}
	}
	pub fn new(name: &str, health: i32, p: EntityType) -> EntityData {
		Self::new_empty(p)
			.with(NameComponent(String::from(name)))
			.with(HealthComponent(health))
	}
	pub fn with<T: 'static>(mut self, component: T) -> Self
			where T: Clone + Component {
		self.contents.insert(component);
		self
	}
	pub fn with_option<T: 'static>(mut self, component: Option<T>) -> Self
			where T: Clone + Component {
		if let Some(c) = component {
			self.contents.insert(c);
		}
		self
	}
	pub fn add<T: 'static>(&mut self, component: T) -> ()
			where T: Clone + Component {
		self.contents.insert(component);
	}
	pub fn add_option<T: 'static>(&mut self, component: Option<T>) -> ()
			where T: Clone + Component {
		if let Some(c) = component {
			self.contents.insert(c);
		}
	}
	#[allow(dead_code)]
	pub fn contains<T: 'static>(&self) -> bool
			where T: Clone + Component {
		self.contents.contains::<T>()
	}
	pub fn remove<T: 'static>(&mut self) -> Option<T>
			where T: Clone + Component {
		let content = self.contents.remove::<T>();
		if let Some(c) = content {
			Some(c)
		} else {
			None
		}
	}
	fn allowed<T: 'static>(&self) -> bool
			where T: Clone + Component {
		match T::purpose() {
			ComponentType::Creature => self.purpose == EntityType::Creature,
			ComponentType::Unanimate => self.purpose == EntityType::Unanimate,
			ComponentType::Shared => true
		}
	}
}
