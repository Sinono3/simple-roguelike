pub type CreatureId = usize;

use crate::features::Feature;

#[derive(Clone)]
pub struct Creature {
	pub name: String,
	pub health: i32,
	pub damage: i32,
	pub features: Vec<Feature>
}