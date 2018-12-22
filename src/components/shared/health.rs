use specs::prelude::*;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Health(pub i32);

impl Health {
    pub fn is_alive(&self) -> bool {
        self.0 > 0
    }
    pub fn has_died(&self) -> bool {
        !self.is_alive()
    }
}

#[derive(Component, Debug, Default)]
#[storage(DenseVecStorage)]
pub struct Hit(pub Option<Entity>);
