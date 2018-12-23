use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs::error::NoError;
use specs::saveload::{Marker, ConvertSaveload};

#[derive(Component, Debug, Default, Clone, Deserialize, Serialize)]
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

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct Affected(pub Entity);

#[derive(Serialize, Deserialize)]
pub struct AffectedData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for Affected
    where for<'de> M: Deserialize<'de>,
{
    type Data = AffectedData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>
    {
        let marker = ids(self.0).unwrap();
        Ok(AffectedData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>
    {
        let entity = ids(data.0).unwrap();
        Ok(Affected(entity))
    }
}
