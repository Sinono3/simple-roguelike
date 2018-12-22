use specs::prelude::*;

use crate::components::creature::{Playable, Attack};
use crate::components::shared::*;
use crate::components::unanimate::Wieldable;

#[derive(Component, Debug, Default, Deserialize, Serialize)]
#[storage(NullStorage)]
pub struct AggressiveBehaviour;

pub struct AggressionSystem;
impl<'a> System<'a> for AggressionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Playable>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, AggressiveBehaviour>,
        ReadStorage<'a, Attack>,
        ReadStorage<'a, Wieldable>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Hit>,
    );

    fn run(&mut self, (entities, player_s, name_s, aggressive_s, attack_s, wieldable_s, mut health_s, mut hit_s): Self::SystemData) {
        use specs::Join;

        let target = (&entities, &player_s).join().nth(0).unwrap().0; // only player for now.
        let (target_name, target_health) =
        (
            &name_s.get(target).unwrap().get(),
            health_s.get_mut(target).unwrap()
        );

        for (entity, _, name, attack) in (&entities, &aggressive_s, &name_s, &attack_s).join() {
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
