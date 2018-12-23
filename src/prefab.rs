/*use specs::saveload::U64Marker;
use specs::Component;

use anymap::{Map, any::CloneAny};
use crate::shared::{*};
use crate::creature::{*, AttackData, NeutralData};
use crate::unanimate::{*, OwnedData};

type CloneMap = Map<CloneAny>;

#[derive(Clone)]
pub struct Prefab {
	contents: CloneMap
}
impl Prefab {
	pub fn new_empty() -> Prefab {
		Prefab {
			contents: Map::<CloneAny>::new()
		}
	}
	pub fn new(name: &str, health: i32) -> Prefab {
		Self::new_empty()
			.with(Name::new(name, false))
			.with(Health(health))
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
}
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

impl<'de> Deserialize<'de> for Prefab {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Name, Health, Components };

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
                        formatter.write_str("`name`, `health` or `components`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "name" => Ok(Field::Name),
                            "health" => Ok(Field::Health),
                            "components" => Ok(Field::Components),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PrefabVisitor;

		#[derive(Deserialize)]
		#[serde(rename_all = "lowercase")]
		enum Component {
			Aggressive(AggressiveBehaviour),
			Attack(AttackData),
			Neutral(NeutralData<U64Marker>),
			Health(Health),
			Name(Name),
			Owned(OwnedData<U64Marker>),
			Salable(Salable),
			Wieldable(Wieldable),
		}

        impl<'de> Visitor<'de> for PrefabVisitor {
            type Value = Prefab;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Prefab")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Prefab, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name = None;
                let mut health = None;
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
                let components = components.ok_or_else(|| de::Error::missing_field("components"))?;

				let mut data = Prefab::new(name, health);
				for c_type in components {
					match c_type {
						Component::Aggressive(c) => data.add(c),
						Component::Attack(c) => data.add(c),
						Component::Neutral(c) => data.add(c),
						Component::Health(c) => data.add(c),
						Component::Name(c) => data.add(c),
						Component::Owned(c) => data.add(c),
						Component::Salable(c) => data.add(c),
						Component::Wieldable(c) => data.add(c),
					}
				}
				Ok(data)
            }
        }

        const FIELDS: &'static [&'static str] = &["name", "health", "components"];
        deserializer.deserialize_struct("entity", FIELDS, PrefabVisitor)
    }
}
*/
