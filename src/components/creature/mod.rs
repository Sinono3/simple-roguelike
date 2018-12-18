mod attack;
mod aggression;
// All systems will be in this module.
pub mod systems {
	pub use super::aggression::aggression;
}

pub use self::attack::AttackComponent;
pub use self::aggression::{AggressionComponent};
