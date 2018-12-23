use specs::storage::GenericReadStorage;
use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs::error::NoError;
use specs::saveload::{Marker, ConvertSaveload};

use crate::unanimate::Wieldable;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct Attack {
	pub strength: i32,
	pub wielding: Option<Entity>
}

impl Attack {
	pub fn damage<T: GenericReadStorage<Component = Wieldable>>(&self, wieldable_s: &T) -> i32 {
		if let Some(wield_id) = self.wielding {
			if let Some(wieldable) = wieldable_s.get(wield_id) {
				return self.strength + wieldable.damage;
			}
			// if they're here they no longer have their weapon... zoinks!
			// this shouldn't even happen, instead it should have been
			// covered if their weapon ever broke, was stolen or they dropped it.
			eprintln!("The wieldable item should exist, but it doesn't.")
		}
		self.strength
	}
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AttackData<M> {
	strength: i32,
    wielding: Option<M>
}
impl<M: Marker + Serialize> ConvertSaveload<M> for Attack
    where for<'de> M: Deserialize<'de>,
{
    type Data = AttackData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>
    {
        if let Some(wielding) = self.wielding {
            let marker = ids(wielding).unwrap();

            Ok(AttackData {
				strength: self.strength,
                wielding: Some(marker)
            })
        } else {
            Ok(AttackData {
				strength: self.strength,
                wielding: None
            })
        }
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>
    {
        if let Some(wielding) = data.wielding {
            let entity = ids(wielding).unwrap();
            Ok(Attack {
				strength: data.strength,
                wielding: Some(entity)
            })
        } else {
            Ok(Attack {
				strength: data.strength,
                wielding: None
            })
        }
    }
}
