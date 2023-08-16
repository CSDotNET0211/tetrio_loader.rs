use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
	Start,
	End,
	Full,
	Keydown,
	Keyup,
	Targets,
	Ige,
}

impl Serialize for EventType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("EventType", 7)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct EventTypeVisitor;

impl<'de> Visitor<'de> for EventTypeVisitor {
	type Value = EventType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("EventType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"start" => Ok(EventType::Start),
			"end" => Ok(EventType::End),
			"full" => Ok(EventType::Full),
			"keydown" => Ok(EventType::Keydown),
			"keyup" => Ok(EventType::Keyup),
			"targets" => Ok(EventType::Targets),
			"ige" => Ok(EventType::Ige),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for EventType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(EventTypeVisitor)
	}
}
