use specs::prelude::*;

use crossterm::style::{Color, style};

use crate::creature::Attack;
use crate::shared::{Name, Health, Affected};
use crate::unanimate::{Wieldable, Owned};

#[derive(Component, Debug, Default, Clone, Deserialize, Serialize)]
#[storage(NullStorage)]
pub struct Playable;

pub struct PlayabilitySystem;
impl<'a> System<'a> for PlayabilitySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Playable>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, Attack>,
        WriteStorage<'a, Wieldable>,
        WriteStorage<'a, Affected>,
        WriteStorage<'a, Owned>,
    );

    fn run(&mut self, (entities, playable_s, name_s, mut health_s, mut attack_s, wieldable_s, mut hit_s, mut owned_s): Self::SystemData) {
        enum Command {
            Affected(Entity),
            Take(Entity),
            Wield(Entity),
            Status,
        }

        use specs::Join;
        for (entity, _, name) in (&entities, &playable_s, &name_s).join() {
            let mut action_points = 2;

            while action_points > 0 {
                let command =
                {
                    use std::io;
                    use std::io::prelude::*;
                    let mut input = String::new();

                    loop {
                        print!("{}",
                            style(format!("What will {} do? ",
                                name.get()
                            )).with(Color::DarkGreen)
                        );
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();
                    	let parts: Vec<&str> = input.trim().split(' ').collect();

                        match parts[0] {
                            "attack" | "hit" if parts.len() > 1 => {
                                if let Some(f) = find(&name_s, &entities, parts[1]) {
                                    break Command::Affected(f);
                                } else {
                                    println!("{}",
                                        style("Please write a correct target. ex: goblin")
                                            .with(Color::DarkRed)
                                    );
                                }
                            }
                            "attack" | "hit" => println!("{}",
                                    style("Please write a target. ex: goblin")
                                        .with(Color::DarkRed)
                                ),

                            "take" | "steal" => {
                                if let Some(item) = find(&name_s, &entities, parts[1]) {
                                    break Command::Take(item);
                                } else {
                                    println!("{}",
                                        style("Please write an existing item.")
                                            .with(Color::DarkRed)
                                    );
                                }
                            }
                            "take" | "steal" => println!("{}",
                                    style("Please write an item. ex: goldenring")
                                    .with(Color::DarkRed)
                                ),

                            "wield" => {
                                if let Some(item) = find(&name_s, &entities, parts[1]) {
                                    break Command::Wield(item);
                                } else {
                                    println!("{}",
                                        style("Please write an existing weapon.")
                                            .with(Color::DarkRed)
                                    );
                                }
                            }
                            "wield" => println!("{}",
                                    style("Please write an item. ex: rustysword")
                                    .with(Color::DarkRed)
                                ),

                            "status" => break Command::Status,

                            _ => println!("{}",
                                    style("Please write an existing command.")
                                    .with(Color::DarkRed)
                                ),
                        }

                        input.clear();
                    }
                };

                match command {
                    Command::Affected(target) => {
                        let (target_name, target_health) = {(
                            &name_s.get(target).unwrap().get(),
                            health_s.get_mut(target).unwrap()
                        )};

                        let damage = attack_s.get(entity).unwrap().damage(&wieldable_s);
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
                            )).with(Color::Green)
                        );

                        if target_health.has_died() {
                            // TODO: Better error handling.
                            entities.delete(target);
                            println!
                            (
                                "{}",
                                style(format!("{} has died!",
                                    target_name
                                )).with(Color::Green)
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

                        action_points -= 2;
                    }
                    Command::Take(e) => {
                        if let Some(owned) = owned_s.get_mut(e) {
                            let owner_name = name_s.get(owned.0).unwrap().get();

                            // temporary!
                            if let Some(att) = attack_s.get_mut(owned.0) {
                                att.wielding = None;
                            }

                            owned.0 = entity;

                            println!("{}",
                                style(format!("{} has stolen {} from {}!",
                                    name.get(),
                                    name_s.get(e).unwrap().get(),
                                    owner_name
                                )).with(Color::Magenta)
                            );
                            // call maintain.
                        } else {
                            println!("{}",
                                style(format!("{} has taken {}.",
                                    name.get(),
                                    name_s.get(e).unwrap().get()
                                )).with(Color::Magenta)
                            );
                            owned_s.insert(e, Owned(entity));
                        }

                        action_points -= 1;
                    }
                    Command::Wield(e) => {
                        if let Some(owned) = owned_s.get_mut(e) {
                            if owned.0 == entity {
                                println!("{}",
                                    style(format!("{} has equipped the {}.",
                                        name.get(),
                                        name_s.get(e).unwrap().get()
                                    )).with(Color::Magenta)
                                );

                                // should support un-attack creatures with if-let, no unwrap.
                                attack_s.get_mut(entity).unwrap().wielding = Some(e);

                                action_points -= 1;
                                continue;
                            }
                        }
                        println!("{}",
                            style(format!("{} doesn't own that!", name.get()))
                                .with(Color::DarkRed)
                        );
                    }
                    Command::Status => {}
                }
            }
        }
    }
}

fn find(name_s: &ReadStorage<Name>, entities: &Entities, name: &str) -> Option<Entity> {
    let mut found = None;
    for ent in entities.join() {
        if name_s.get(ent).unwrap().raw() == name {
            found = Some(ent);
        }
    }
    found
}
