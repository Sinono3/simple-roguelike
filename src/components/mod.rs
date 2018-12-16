mod name;
mod health;
mod attack;
mod aggression;
// All systems will be in this module.
pub mod systems {
	pub use super::aggression::aggression;
}
pub mod map;

// Component importing.
pub use self::health::HealthComponent;
pub use self::name::NameComponent;
pub use self::attack::AttackComponent;
pub use self::aggression::{AggressionComponent};
