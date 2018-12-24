use specs::storage::GenericReadStorage;
use specs::prelude::*;

use crate::unanimate::{Owned, Tradeable};

#[derive(Component, Debug, Clone)]
#[storage(HashMapStorage)]
pub struct Trader {
	pub interest: i32
}

impl Trader {
	pub fn stock(trader: Entity, entities: &Entities, owned_s: &WriteStorage<Owned>,
			tradeable_s: &ReadStorage<Tradeable>) -> Vec<Entity> {

		let mut stock: Vec<Entity> = Vec::new();

		for (e, o, _) in (entities, owned_s, tradeable_s).join() {
			if o.0 == trader {
				stock.push(e);
			}
		}
		stock
	}
	pub fn interest(&self, price: i32) -> i32 {
		price * (1 + self.interest / 100)
	}
}
