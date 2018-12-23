use std::fs::File;
use specs::saveload::{DeserializeComponents, MarkedBuilder, SerializeComponents, U64Marker, U64MarkerAllocator};

// Here we create a system that lets us access the entities to serialize.
struct SaveSystem;

impl<'a> System<'a> for SaveSystem {
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
        ReadStorage<'a, Tradeable>,
        ReadStorage<'a, Wieldable>,
        ReadStorage<'a, Playable>,
        ReadStorage<'a, U64Marker>,
    );

    fn run(&mut self, (ents, agg, neu, att, health, aff, nam, own, sal, wiel, play, mark): Self::SystemData) {

        let mut write = File::create("resources/saves/save0.json").unwrap();

        let mut ser = serde_json::ser::Serializer::pretty(write);
        SerializeComponents::<NoError, U64Marker>::serialize(
            &(&nam, &health, &att, &agg, &neu, &aff, &own, &sal, &wiel, &play),
            &ents,
            &mark,
            &mut ser,
        ).unwrap_or_else(|e| eprintln!("Error: {}", e));
        println!("Saved successfully to slot 0!");
    }
}
