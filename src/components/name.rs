use super::Component;

#[derive(Clone)]
pub struct NameComponent(pub String);

impl From<NameComponent> for Component {
	fn from(component: NameComponent) -> Self { Component::Name }
}
