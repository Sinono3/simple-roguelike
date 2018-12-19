mod attack;
mod aggressive;
mod neutral;
// All systems will be in this module.
pub mod systems {
	pub use super::aggressive::aggressive;
	pub use super::neutral::neutral;
}

pub use self::attack::AttackComponent;
pub use self::aggressive::AggressiveComponent;
pub use self::neutral::NeutralComponent;
