extern crate crossterm;
extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use crossterm::terminal::*;
use crossterm::style::{Color, style};

use specs::prelude::*;
use specs::saveload::{U64Marker, U64MarkerAllocator, MarkedBuilder};

mod creature;
mod shared;
mod unanimate;

use crate::creature::*;
use crate::unanimate::*;
use crate::shared::*;

#[allow(unused_variables)]
fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let mut world = World::new();
	world.register::<AggressiveBehaviour>();
	world.register::<NeutralBehaviour>();
	world.register::<Combatant>();
	world.register::<Health>();
	world.register::<Affected>();
	world.register::<Name>();
	world.register::<Owned>();
	world.register::<Tradeable>();
	world.register::<Trader>();
	world.register::<Wieldable>();
	world.register::<Playable>();
    world.register::<U64Marker>();

	world.add_resource(U64MarkerAllocator::new());

	let rusty_sword = world.create_entity()
		.with(Name::new("rustysword", false))
		.with(Health(18))
		.with(Wieldable {
			damage: 2
		})
		.with(Tradeable {
			worth: 10
		})
		.marked::<U64Marker>()
		.build();

	let blood_sword = world.create_entity()
		.with(Name::new("bloodsword", true))
		.with(Health(90))
		.with(Wieldable {
			damage: 8
		})
		.with(Tradeable {
			worth: 470
		})
		.marked::<U64Marker>()
		.build();

	let servant = world.create_entity()
		.with(Name::new("Wigfrid", true))
		.with(Health(20))
		.with(Combatant {
			strength: 2,
			wielding: None
		})
		.with(Playable)
		.marked::<U64Marker>()
		.build();

	/*let homeless = world.create_entity()
		.with(Name::new("Mondhart", true))
		.with(Health(35))
		.with(Combatant {
			strength: 6,
			wielding: None
		})
		.with(Playable)
		.marked::<U64Marker>()
		.build();*/

	let goblin = world
		.create_entity()
		.with(Name::new("goblin", false))
		.with(Health(12))
		.with(Combatant {
			strength: 1,
			wielding: None
		})
		.with(AggressiveBehaviour)
		.marked::<U64Marker>()
		.build();

	let merchant = world
		.create_entity()
		.with(Name::new("merchant", false))
		.with(Health(38))
		.with(Combatant {
			strength: 1,
			wielding: None
		})
		.with(NeutralBehaviour::new())
		.with(Trader { interest: 15 })
		.marked::<U64Marker>()
		.build();

	introduction();

	// owning example (very simple)
	world.exec(|mut data: WriteStorage<Owned>| {
		// TODO: Better error handling.
		data.insert(rusty_sword, Owned(servant)).unwrap();
		data.insert(blood_sword, Owned(merchant)).unwrap();
	});
	// call maintain.

	// auto-wielding example.
	world.exec(|(mut att, own): (WriteStorage<Combatant>, ReadStorage<Owned>)| {
		if let Some(owned) = own.get(rusty_sword) {
			att.get_mut(owned.0).unwrap().wielding = Some(rusty_sword);
		}
		if let Some(owned) = own.get(blood_sword) {
			att.get_mut(owned.0).unwrap().wielding = Some(blood_sword);
		}
	});
	// call maintain.

	use specs::RunNow;

	let mut play = PlayabilitySystem;
	let mut aggro = AggressionSystem;
	let mut neutral = NeutralitySystem;
	// TODO: World and stolen maintain system.

	loop {
		play.run_now(&world.res);
		world.maintain();
		aggro.run_now(&world.res);
		world.maintain();
		neutral.run_now(&world.res);
		world.maintain();
	}
}
fn introduction() -> () {
	let line = style("##########################################").with(Color::DarkYellow);
	println!("{}", line);
	println!("{}", style("######### Simple Rusty Roguelike #########").with(Color::DarkYellow));
	println!("{}", line);

	println!("{}", style("
## You are Wigfrid, a servant at the Motvasser castle, your master,
Dr. Arkan has thrown you into the castle's basement. You must escape
before his horrible creations consume you alive!\n")
			.with(Color::Green));

	println!("{}", style("Type 'help' to see the available commands.")
			.with(Color::DarkGreen));
}
