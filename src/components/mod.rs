pub mod creature;
pub mod unanimate;
pub mod shared;

mod entity_allocator;
mod entity_data;
mod entity_map;

pub use self::entity_data::EntityData;
pub use self::entity_allocator::EntityAllocator;
pub use self::entity_map::EntityMap;

pub type Entity = usize;

#[derive(Clone, PartialEq)]
pub enum ComponentType {
    Creature,
    Unanimate,
    Shared,
}
#[derive(Clone, PartialEq)]
pub enum EntityType {
    Creature,
    Unanimate,
}

pub trait Component: Clone {
    fn purpose() -> ComponentType;
}
pub type ComponentMap<T> = Vec<Option<T>>;
