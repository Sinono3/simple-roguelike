use anymap::{Map, any::CloneAny};
use crate::components::{NameComponent, HealthComponent};

type CloneMap = Map<CloneAny>;

#[derive(Clone)]
pub struct CreatureData {
	contents: CloneMap,
}
impl CreatureData {
	pub fn new_empty() -> CreatureData {
		CreatureData {
			contents: CloneMap::new()
		}
	}
	pub fn new(name: &str, health: i32) -> CreatureData {
		Self::new_empty()
			.with(NameComponent(String::from(name)))
			.with(HealthComponent(health))
	}
	pub fn with<T: 'static>(mut self, component: T) -> Self where T: Clone {
		self.contents.insert(component);
		self
	}
	pub fn with_option<T: 'static>(mut self, component: Option<T>) -> Self where T: Clone {
		if let Some(c) = component {
			self.contents.insert(c);
		}
		self
	}
	#[allow(dead_code)]
	pub fn contains<T: 'static>(&self) -> bool where T: Clone {
		self.contents.contains::<T>()
	}
	pub fn remove<T: 'static>(&mut self) -> Option<T> where T: Clone {
		let content = self.contents.remove::<T>();
		if let Some(c) = content {
			Some(c)
		} else {
			None
		}
	}
}
