use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs::error::NoError;
use specs::saveload::{Marker, ConvertSaveload};

#[derive(Component, Debug, Clone)] // Deserialize, Serialize
#[storage(DenseVecStorage)]
pub struct Owned(pub Entity);

#[derive(Clone, Serialize, Deserialize)]
pub struct OwnedData<M>(M);

impl<M: Marker + Serialize> ConvertSaveload<M> for Owned
    where for<'de> M: Deserialize<'de>,
{
    type Data = OwnedData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>
    {
        let marker = ids(self.0).unwrap();
        Ok(OwnedData(marker))
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>
    {
        let entity = ids(data.0).unwrap();
        Ok(Owned(entity))
    }
}
