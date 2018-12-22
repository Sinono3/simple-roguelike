use specs::storage::GenericReadStorage;
use specs::prelude::*;

use crate::components::unanimate::Wieldable;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Attack {
	pub strength: i32,
	pub wielding: Option<Entity>
}

impl Attack {
	pub fn damage<T: GenericReadStorage<Component = Wieldable>>(&self, wieldable_s: &T) -> i32 {
		if let Some(wield_id) = self.wielding {
			if let Some(wieldable) = wieldable_s.get(wield_id) {
				return self.strength + wieldable.damage;
			}
			// if they're here they no longer have their weapon... zoinks!
			// this shouldn't even happen, instead it should have been
			// covered if their weapon ever broke, was stolen or they dropped it.
			eprintln!("The wieldable item should exist, but it doesn't.")
		}
		self.strength
	}
}
