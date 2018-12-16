use super::Component;

#[derive(Clone)]
pub struct HealthComponent(pub i32);

impl From<HealthComponent> for Component {
	fn from(component: HealthComponent) -> Self { Component::Health }
}
