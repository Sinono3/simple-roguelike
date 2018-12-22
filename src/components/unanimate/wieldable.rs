use specs::{Component, DenseVecStorage};

#[derive(Component, Debug, Deserialize, Serialize)]
#[storage(DenseVecStorage)]
pub struct Wieldable {
	pub damage: i32
}
