mod name;
mod health;
mod owner;
pub mod systems {

}

pub use self::health::HealthComponent;
pub use self::name::NameComponent;
pub use self::owner::OwnerComponent;
