mod attack;
mod aggression;
mod neutral;
// All systems will be in this module.
pub mod systems {
	pub use super::aggression::aggression;
	pub use super::neutral::neutral;
}

pub use self::attack::AttackComponent;
pub use self::aggression::AggressionComponent;
pub use self::neutral::NeutralComponent;
