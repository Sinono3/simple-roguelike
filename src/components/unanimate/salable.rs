use specs::{Component, DenseVecStorage};

#[derive(Component, Debug)] // Deserialize, Serialize
#[storage(DenseVecStorage)]
pub struct Salable {
	pub worth: i32
}
