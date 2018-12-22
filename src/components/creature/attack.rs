use specs::prelude::*;

use crate::components::unanimate::Wieldable;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Attack {
	pub strength: i32,
	pub wielding: Option<Entity>
}

impl Attack {
	pub fn damage(&self, wieldable_s: &ReadStorage<Wieldable>) -> i32 {
		if let Some(wield_id) = self.wielding {
			if let Some(wieldable) = wieldable_s.get(wield_id) {
				return self.strength + wieldable.damage;
			}
			// if they're here they no longer have their weapon... zoinks!
		}
		self.strength
	}
	pub fn damage_mut(&self, wieldable_s: &WriteStorage<Wieldable>) -> i32 {
		if let Some(wield_id) = self.wielding {
			if let Some(wieldable) = wieldable_s.get(wield_id) {
				return self.strength + wieldable.damage;
			}
			// if they're here they no longer have their weapon... zoinks!
		}
		self.strength
	}
}
