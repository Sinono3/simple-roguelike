use std::any::TypeId;

mod name;
mod health;
mod attack;
mod aggression;
pub mod systems;

pub use self::health::HealthComponent;
pub use self::name::NameComponent;
pub use self::attack::AttackComponent;
pub use self::aggression::{AggressionComponent};

pub const NAME_COMPONENT_ID: TypeId = TypeId::of::<NameComponent>();
pub const HEALTH_COMPONENT_ID: TypeId = TypeId::of::<HealthComponent>();
pub const ATTACK_COMPONENT_ID: TypeId = TypeId::of::<AttackComponent>();
pub const AGGRESSION_COMPONENT_ID: TypeId = TypeId::of::<AggressionComponent>();

#[derive(Clone, Debug)]
pub enum Component {
	Name,
	Health,
	Attack,
	Aggression,
}
impl Component {
	pub fn compare(&self, other: &Component) -> bool {
    	std::mem::discriminant(self) == std::mem::discriminant(other)
	}
}
