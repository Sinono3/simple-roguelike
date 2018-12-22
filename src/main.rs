extern crate crossterm;
extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ron;

use crossterm::terminal::*;
use crossterm::style::{Color, style};

use specs::prelude::*;

mod commands;
mod components;

use crate::components::creature::*;
use crate::components::unanimate::*;
use crate::components::shared::*;

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let mut world = World::new();
	world.register::<AggressiveBehaviour>();
	world.register::<NeutralBehaviour>();
	world.register::<Attack>();
	world.register::<Health>();
	world.register::<Hit>();
	world.register::<Name>();
	world.register::<Owned>();
	world.register::<Salable>();
	world.register::<Wieldable>();
	world.register::<Playable>();

	let rusty_sword = world.create_entity()
		.with(Name::new("rusty_sword"))
		.with(Health(18))
		.with(Wieldable {
			damage: 2
		})
		.build();

	let blood_sword = world.create_entity()
		.with(Name::new("blood_sword"))
		.with(Health(90))
		.with(Wieldable {
			damage: 8
		})
		.build();

	let warrior = world.create_entity()
		.with(Name::new("Wigfrid"))
		.with(Health(20))
		.with(Attack {
			strength: 2,
			wielding: Some(rusty_sword)
		})
		.with(Playable)
		.build();

	let goblin = world
		.create_entity()
		.with(Name::new("goblin"))
		.with(Health(12))
		.with(Attack {
			strength: 1,
			wielding: None
		})
		.with(AggressiveBehaviour)
		.build();

	let merchant = world
		.create_entity()
		.with(Name::new("merchant"))
		.with(Health(38))
		.with(Attack {
			strength: 1,
			wielding: Some(blood_sword)
		})
		.with(NeutralBehaviour::default())
		.build();

	introduction();

	world.exec(|mut data: WriteStorage<Owned>| {
		// TODO: Better error handling.
		data.insert(rusty_sword, Owned(warrior));
		data.insert(blood_sword, Owned(merchant));
	});

	use specs::RunNow;

	let mut play = PlayabilitySystem;
	let mut aggro = AggressionSystem;
	let mut neutral = NeutralitySystem;

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
