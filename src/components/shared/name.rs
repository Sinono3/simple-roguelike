#[derive(Clone, Debug)]
pub struct NameComponent(pub String);

use crate::components::{Component, ComponentPurpose};
impl Component for NameComponent {
	fn purpose() -> ComponentPurpose { ComponentPurpose::Shared }
}
