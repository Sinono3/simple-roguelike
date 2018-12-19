use crate::util::anymap::{Map, any::CloneAny};
use super::{Component, ComponentType, EntityType};
//use super::shared::{NameComponent, HealthComponent};
use super::shared::*;
use super::creature::*;
use super::unanimate::*;

type CloneMap = Map<CloneAny>;

#[derive(Clone)]
pub struct EntityData {
	contents: CloneMap,
	purpose: EntityType,
}
#[allow(dead_code)]
impl EntityData {
	pub fn new_empty(p: EntityType) -> EntityData {
		EntityData {
			contents: Map::<CloneAny>::new(),
			purpose: p
		}
	}
	pub fn new(name: &str, health: i32, p: EntityType) -> EntityData {
		Self::new_empty(p)
			.with(NameComponent(String::from(name)))
			.with(HealthComponent(health))
	}
	pub fn with<T: 'static>(mut self, component: T) -> Self
			where T: Clone + Component {
		self.contents.insert(component);
		self
	}
	pub fn with_option<T: 'static>(mut self, component: Option<T>) -> Self
			where T: Clone + Component {
		if let Some(c) = component {
			self.contents.insert(c);
		}
		self
	}
	pub fn add<T: 'static>(&mut self, component: T) -> ()
			where T: Clone + Component {
		self.contents.insert(component);
	}
	pub fn add_option<T: 'static>(&mut self, component: Option<T>) -> ()
			where T: Clone + Component {
		if let Some(c) = component {
			self.contents.insert(c);
		}
	}
	#[allow(dead_code)]
	pub fn contains<T: 'static>(&self) -> bool
			where T: Clone + Component {
		self.contents.contains::<T>()
	}
	pub fn remove<T: 'static>(&mut self) -> Option<T>
			where T: Clone + Component {
		let content = self.contents.remove::<T>();
		if let Some(c) = content {
			Some(c)
		} else {
			None
		}
	}
	fn allowed<T: 'static>(&self) -> bool
			where T: Clone + Component {
		match T::purpose() {
			ComponentType::Creature => self.purpose == EntityType::Creature,
			ComponentType::Unanimate => self.purpose == EntityType::Unanimate,
			ComponentType::Shared => true
		}
	}
}
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

impl<'de> Deserialize<'de> for EntityData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Name, Health, Type, Components };

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`name`, `health`, `type` or `components`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "health" => Ok(Field::Health),
                            "type" => Ok(Field::Type),
                            "components" => Ok(Field::Components),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct EntityDataVisitor;

		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Component {
			Aggressive(AggressiveComponent),
			Attack(AttackComponent),
			Neutral(NeutralComponent),
			Health(HealthComponent),
			Name(NameComponent),
			Owner(OwnerComponent),
			Owned(OwnedComponent),
			Salable(SalableComponent),
			Wieldable(WieldableComponent),
		}

        impl<'de> Visitor<'de> for EntityDataVisitor {
            type Value = EntityData;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct EntityData")
            }

            fn visit_map<V>(self, mut map: V) -> Result<EntityData, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut health = None;
                let mut entity_type = None;
                let mut components: Option<Vec<Component>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(map.next_value()?);
                        }
                        Field::Health => {
                            if health.is_some() {
                                return Err(de::Error::duplicate_field("health"));
                            }
                            health = Some(map.next_value()?);
                        }
						Field::Type => {
                            if entity_type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            entity_type = Some(map.next_value()?);
                        }
						Field::Components => {
                            if components.is_some() {
                                return Err(de::Error::duplicate_field("components"));
                            }
                            components = Some(map.next_value()?);
                        }
                    }
                }
                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let health = health.ok_or_else(|| de::Error::missing_field("health"))?;
                let entity_type = entity_type.ok_or_else(|| de::Error::missing_field("type"))?;
                let components = components.ok_or_else(|| de::Error::missing_field("components"))?;

				let mut data = EntityData::new(name, health, entity_type);
				for c_type in components {
					match c_type {
						Component::Aggressive(c) => data.add(c),
						Component::Attack(c) => data.add(c),
						Component::Neutral(c) => data.add(c),
						Component::Health(c) => data.add(c),
						Component::Name(c) => data.add(c),
						Component::Owner(c) => data.add(c),
						Component::Owned(c) => data.add(c),
						Component::Salable(c) => data.add(c),
						Component::Wieldable(c) => data.add(c),
					}
				}
				Ok(data)
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "health", "type", "components"];
        deserializer.deserialize_struct("entity", FIELDS, EntityDataVisitor)
    }
}
