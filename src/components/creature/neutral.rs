use specs::prelude::*;

use crate::components::creature::Attack;
use crate::components::shared::{Name, Health, Hit};
use crate::components::unanimate::Wieldable;

#[derive(Component, Debug, Default)] // Deserialize, Serialize
#[storage(DenseVecStorage)]
pub struct NeutralBehaviour {
    target: Option<Entity>
}
pub struct NeutralitySystem;
impl<'a> System<'a> for NeutralitySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, NeutralBehaviour>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Hit>,
        ReadStorage<'a, Attack>,
        ReadStorage<'a, Wieldable>
    );

    fn run(&mut self, (entities, mut neutral_s, name_s, mut health_s, mut hit_s, attack_s, wieldable_s): Self::SystemData) {
        use specs::Join;

        for (entity, mut neutral, name, attack) in (&entities, &mut neutral_s, &name_s, &attack_s).join() {
            if let Some(hit) = hit_s.get(entity) {
                neutral.target = hit.0;
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
                hit_s.insert(target, Hit(Some(entity)));

                println!
                (
                    "{} hit {} for {} damage!",
                    name.get(),
                    target_name,
                    damage
                );

                if target_health.has_died() {
                    // TODO: Better error handling.
                    entities.delete(target);
                    println!
                    (
                        "{} has died!",
                        target_name
                    );
                } else {
                    println!
                    (
                        "{} now has {} hitpoints.",
                        target_name,
                        target_health.0
                    );
                }
            }
        }
    }
}
