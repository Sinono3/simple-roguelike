use crate::components::{Component, ComponentType};

#[derive(Clone, Debug)]
pub struct SalableComponent {
	pub worth: i32
}
impl Component for SalableComponent {
	fn purpose() -> ComponentType { ComponentType::Unanimate }
}
impl SalableComponent {
	pub fn apply_interest(&self, interest: i32) -> i32 {
		self.worth * (100 / interest)
	}
}
