use serde::{Serialize, Deserialize};
use specs::prelude::*;
use specs::error::NoError;
use specs::saveload::{Marker, ConvertSaveload};

use crossterm::style::{Color, style};

use crate::creature::Attack;
use crate::shared::{Name, Health, Affected};
use crate::unanimate::Wieldable;

#[derive(Component, Debug, Clone)]
#[storage(DenseVecStorage)]
pub struct NeutralBehaviour {
    target: Option<Entity>
}
impl NeutralBehaviour {
    pub fn new() -> Self {
        NeutralBehaviour {
            target: None
        }
    }
}

pub struct NeutralitySystem;
impl<'a> System<'a> for NeutralitySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, NeutralBehaviour>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Affected>,
        ReadStorage<'a, Attack>,
        ReadStorage<'a, Wieldable>
    );

    fn run(&mut self, (entities, mut neutral_s, name_s, mut health_s, mut hit_s, attack_s, wieldable_s): Self::SystemData) {
        use specs::Join;

        for (entity, mut neutral, name, attack) in (&entities, &mut neutral_s, &name_s, &attack_s).join() {
            if let Some(hit) = hit_s.get(entity) {
                neutral.target = Some(hit.0);
            }

            if let Some(target) = neutral.target {
                let (target_name, target_health) =
                (
                    &name_s.get(target).unwrap().get(),
                    health_s.get_mut(target).unwrap()
                );

                let damage = attack.damage(&wieldable_s);
                target_health.0 -= damage;
                // TODO: Better error handling.
                hit_s.insert(target, Affected(entity));

                println!
                (
                    "{}",
                    style(format!("{} hit {} for {} damage!",
                        name.get(),
                        target_name,
                        damage
                    )).with(Color::Red)
                );

                if target_health.has_died() {
                    // TODO: Better error handling.
                    entities.delete(target);
                    println!
                    (
                        "{}",
                        style(format!("{} has died!",
                            target_name
                        )).with(Color::Red)
                    );
                } else {
                    println!
                    (
                        "{}",
                        style(format!("{} now has {} hitpoints.",
                            target_name,
                            target_health.0
                        )).with(Color::Cyan)
                    );
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NeutralData<M> {
    target: Option<M>
}
impl<M: Marker + Serialize> ConvertSaveload<M> for NeutralBehaviour
    where for<'de> M: Deserialize<'de>,
{
    type Data = NeutralData<M>;
    type Error = NoError;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>
    {
        if let Some(target) = self.target {
            let marker = ids(target).unwrap();

            Ok(NeutralData {
                target: Some(marker)
            })
        } else {
            Ok(NeutralData {
                target: None
            })
        }
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>
    {
        if let Some(target) = data.target {
            let entity = ids(target).unwrap();
            Ok(NeutralBehaviour {
                target: Some(entity)
            })
        } else {
            Ok(NeutralBehaviour {
                target: None
            })
        }
    }
}
