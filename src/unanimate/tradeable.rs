use specs::{Component, DenseVecStorage};

#[derive(Component, Debug, Clone, Deserialize, Serialize)]
#[storage(DenseVecStorage)]
pub struct Tradeable {
	pub worth: i32
}
