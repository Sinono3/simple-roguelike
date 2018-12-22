use specs::{Entity, Component, DenseVecStorage};

#[derive(Component, Debug)] // Deserialize, Serialize
#[storage(DenseVecStorage)]
pub struct Owned(pub Entity);
