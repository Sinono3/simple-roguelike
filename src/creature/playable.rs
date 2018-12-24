use specs::prelude::*;

use crossterm::style::{Color, style};

use crate::creature::{Trader, Combatant};
use crate::shared::{Name, Health, Affected};
use crate::unanimate::{Tradeable, Wieldable, Owned};

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
        WriteStorage<'a, Combatant>,
        WriteStorage<'a, Trader>,
        WriteStorage<'a, Wieldable>,
        ReadStorage<'a, Tradeable>,
        WriteStorage<'a, Affected>,
        WriteStorage<'a, Owned>,
    );

    fn run(&mut self, (entities, playable_s, name_s, mut health_s, mut attack_s, mut trader_s,
                       wieldable_s, tradeable_s, mut affected_s, mut owned_s): Self::SystemData) {
        // TODO: Each action should be put in it's respective system. ex: GiveSystem
        enum Command {
            Hit(Entity),
            Take(Entity),
            Give(Entity, Entity),
            Wield(Entity),
            Trade(Entity),
            Status,
        }

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
                                    break Command::Hit(f);
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

                            "take" | "steal" if parts.len() > 1 => {
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

                            "give" if parts.len() > 2 => {
                                if let Some(item) = find(&name_s, &entities, parts[1]) {
                                    if let Some(receiver) = find(&name_s, &entities, parts[2]) {
                                        break Command::Give(item, receiver);
                                    } else {
                                        println!("{}",
                                            style("Please write an existing creature.")
                                                .with(Color::DarkRed)
                                        );
                                    }
                                } else {
                                    println!("{}",
                                        style("Please write an existing item.")
                                            .with(Color::DarkRed)
                                    );
                                }
                            }
                            "give" if parts.len() > 1 => println!("{}",
                                style("Please write a receiver. ex: mondhart")
                                    .with(Color::DarkRed)
                            ),
                            "give" => println!("{}",
                                style("Please write the gift and receiver. ex: rustysword mondhart")
                                    .with(Color::DarkRed)
                            ),

                            "wield" if parts.len() > 1 => {
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

                            "trade" if parts.len() > 1 => {
                                    if let Some(item) = find(&name_s, &entities, parts[1]) {
                                        break Command::Trade(item);
                                    } else {
                                        println!("{}",
                                            style("Please write an existing trader.")
                                                .with(Color::DarkRed)
                                        );
                                    }
                            }
                            "trade" => println!("{}",
                                style("Please write an trader. ex: merchant")
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
                    Command::Hit(target) => {
                        let (target_name, target_health) = {(
                            &name_s.get(target).unwrap().get(),
                            health_s.get_mut(target).unwrap()
                        )};

                        let damage = attack_s.get(entity).unwrap().damage(&wieldable_s);
                        target_health.0 -= damage;

                        // TODO: Better error handling.
                        affected_s.insert(target, Affected(entity)).unwrap();

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
                            entities.delete(target).unwrap();
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

                            // TODO: Better error handling.
                            affected_s.insert(owned.0, Affected(entity)).unwrap();

                            // temporary!
                            if let Some(att) = attack_s.get_mut(owned.0) {
                                if att.wielding == Some(e) {
                                    att.wielding = None;
                                }
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
                            // TODO: better error handling
                            owned_s.insert(e, Owned(entity)).unwrap();
                        }

                        action_points -= 1;
                    }
                    Command::Give(i, r) => {
                        if let Some(owned) = owned_s.get_mut(i) {
                            if owned.0 == entity {
                                let receiver_name = name_s.get(r).unwrap().get();

                                // TODO: Better error handling.
                                affected_s.insert(owned.0, Affected(entity)).unwrap();

                                // temporary!
                                if let Some(att) = attack_s.get_mut(owned.0) {
                                    if att.wielding == Some(i) {
                                        att.wielding = None;
                                    }
                                }

                                owned.0 = r;

                                println!("{}",
                                    style(format!("{} has given {} to {}!",
                                        name.get(),
                                        name_s.get(i).unwrap().get(),
                                        receiver_name
                                    )).with(Color::Magenta)
                                );

                                action_points -= 1;
                                continue;
                                // call maintain.
                            }
                        }
                        println!("{}",
                            style("You can't give what you don't own!")
                                .with(Color::DarkRed)
                        );
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
                    Command::Trade(e) => {
                        if let Some(trader) = trader_s.get(e) {
                            println!("{}",
                                style("Items on stock:")
                                    .with(Color::Yellow)
                            );

                            let stock = Trader::stock(e, &entities, &owned_s, &tradeable_s);
                            for item in stock {
                                let item_name = name_s.get(item).unwrap().get();
                                let price = trader.interest(tradeable_s.get(item).unwrap().worth);

                                println!("{}",
                                    style(format!("{} costs {} gold.",
                                        item_name,
                                        price
                                    )).with(Color::Yellow)
                                );

                                if yes_no("Would you like to buy this item? ") {
                                    // temporary replacement for item maintaining.
                                    if let Some(att) = attack_s.get_mut(e) {
                                        if att.wielding == Some(item) {
                                            att.wielding = None;
                                        }
                                    }

                                    // TODO: Should check and remove gold from player.

                                    owned_s.get_mut(item).unwrap().0 = entity;

                                    println!("{}",
                                        style(format!("{} has bought {} for {} gold.",
                                            name.get(),
                                            item_name,
                                            price
                                        )).with(Color::Magenta)
                                    );
                                }
                            }
                            action_points -= 1;
                        } else {
                            println!("{}",
                                style("They are not a trader.")
                                    .with(Color::DarkRed)
                            );
                        }
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
fn yes_no(queue: &str) -> bool {
    use std::io;
    use std::io::prelude::*;
    let mut input = String::new();

    loop {
        print!("{}",
            style(queue)
                .with(Color::DarkGreen)
        );
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "y" | "yes" => break true,
            "n" | "no" => break false,
            _ => println!("{}",
                    style("Please answer yes or no.")
                        .with(Color::DarkRed)
                )
        }
        input.clear();
    }
}
