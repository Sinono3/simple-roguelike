extern crate crossterm;
extern crate specs;
#[macro_use]
extern crate specs_derive;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate ron;

use std::fmt;
use crossterm::terminal::*;
use crossterm::style::{Color, style};

use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, U64Marker, U64MarkerAllocator};

mod creature;
mod shared;
mod unanimate;

use crate::creature::*;
use crate::unanimate::*;
use crate::shared::*;

#[derive(Debug)]
enum Combined {
    Ron(ron::ser::Error),
}
impl fmt::Display for Combined {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Combined::Ron(ref e) => write!(f, "{}", e),
        }
    }
}
impl From<ron::ser::Error> for Combined {
    fn from(x: ron::ser::Error) -> Self {
        Combined::Ron(x)
    }
}
impl From<NoError> for Combined {
    fn from(e: NoError) -> Self {
        match e {}
    }
}

fn main() {
	let terminal = terminal();
	terminal.clear(ClearType::All);

	let mut world = World::new();
	world.register::<AggressiveBehaviour>();
	world.register::<NeutralBehaviour>();
	world.register::<Attack>();
	world.register::<Health>();
	world.register::<Affected>();
	world.register::<Name>();
	world.register::<Owned>();
	world.register::<Salable>();
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
		.marked::<U64Marker>()
		.build();

	let blood_sword = world.create_entity()
		.with(Name::new("bloodsword", true))
		.with(Health(90))
		.with(Wieldable {
			damage: 8
		})
		.marked::<U64Marker>()
		.build();

	let servant = world.create_entity()
		.with(Name::new("Wigfrid", true))
		.with(Health(20))
		.with(Attack {
			strength: 2,
			wielding: None
		})
		.with(Playable)
		.marked::<U64Marker>()
		.build();

	let homeless = world.create_entity()
		.with(Name::new("Mondhart", true))
		.with(Health(35))
		.with(Attack {
			strength: 6,
			wielding: None
		})
		.with(Playable)
		.marked::<U64Marker>()
		.build();

	let goblin = world
		.create_entity()
		.with(Name::new("goblin", false))
		.with(Health(12))
		.with(Attack {
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
		.with(Attack {
			strength: 1,
			wielding: None
		})
		.with(NeutralBehaviour::new())
		.marked::<U64Marker>()
		.build();

	introduction();

	// owning example (very simple)
	world.exec(|mut data: WriteStorage<Owned>| {
		// TODO: Better error handling.
		data.insert(rusty_sword, Owned(servant));
		data.insert(blood_sword, Owned(merchant));
	});
	// call maintain.

	// auto-wielding example.
	world.exec(|(mut att, own): (WriteStorage<Attack>, ReadStorage<Owned>)| {
		if let Some(owned) = own.get(rusty_sword) {
			att.get_mut(owned.0).unwrap().wielding = Some(rusty_sword);
		}
		if let Some(owned) = own.get(blood_sword) {
			att.get_mut(owned.0).unwrap().wielding = Some(blood_sword);
		}
	});
	// call maintain.

	use specs::RunNow;

	// Here we create a system that lets us access the entities to serialize.
    struct Serialize;

    impl<'a> System<'a> for Serialize {
        // This SystemData contains the entity-resource, as well as all components that shall be serialized,
        // plus the marker component storage.
        type SystemData = (
            Entities<'a>,
            ReadStorage<'a, AggressiveBehaviour>,
            ReadStorage<'a, NeutralBehaviour>,
            ReadStorage<'a, Attack>,
            ReadStorage<'a, Health>,
            ReadStorage<'a, Affected>,
            ReadStorage<'a, Name>,
            ReadStorage<'a, Owned>,
            ReadStorage<'a, Salable>,
            ReadStorage<'a, Wieldable>,
            ReadStorage<'a, Playable>,
            ReadStorage<'a, U64Marker>,
        );

        fn run(&mut self, (ents, agg, neu, att, health, aff, nam, own, sal, wiel, play, mark): Self::SystemData) {
            // First we need a serializer for the format of choice, in this case the `.ron`-format.
            let mut ser = ron::ser::Serializer::new(Some(Default::default()), true);

            // For serialization we use the [`SerializeComponents`](struct.SerializeComponents.html)-trait's `serialize` function.
            // It takes two generic parameters:
            // * An unbound type -> `NoError` (However, the serialize function expects it to be bound by the `Display`-trait)
            // * A type implementing the `Marker`-trait -> [U64Marker](struct.U64Marker.html) (a convenient, predefined marker)
            //
            // The first parameter resembles the `.join()` syntax from other specs-systems,
            // every component that should be serialized has to be put inside a tuple.
            //
            // The second and third parameters are just the entity-storage and marker-storage, which get `.join()`ed internally.
            //
            // Lastly, we provide a mutable reference to the serializer of choice, which has to have the `serde::ser::Serializer`-trait implemented.
            SerializeComponents::<NoError, U64Marker>::serialize(
                &(&agg, &agg, &neu, &att, &health, &aff, &nam, &own, &sal, &wiel, &play),
                &ents,
                &mark,
                &mut ser,
            ).unwrap_or_else(|e| eprintln!("Error: {}", e));
            // TODO: Specs should return an error which combines serialization
            // and component errors.

            // At this point, `ser` could be used to write its contents to a file, which is not done here.
            // Instead we print the content of this pseudo-file.
            println!("{}", ser.into_output_string());
        }
    }

    // Running the system results in a print to the standard output channel, in `.ron`-format,
    // showing how the serialized dummy entities look like.
    Serialize.run_now(&world.res);

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
