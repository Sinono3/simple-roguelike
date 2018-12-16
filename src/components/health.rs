#[derive(Clone)]
pub struct HealthComponent(pub i32);

impl HealthComponent {
	pub fn heal(&mut self, healing: i32) {
		self.0 += healing;
	}
	pub fn damage(&mut self, damage: i32) {
		self.0 -= damage;
	}
}