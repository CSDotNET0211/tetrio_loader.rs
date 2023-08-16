use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum GarbageType {
	Interaction,
	InteractionConfirm,
	Kev,
	Attack,
}

impl Serialize for GarbageType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("GarbageType", 4)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct GarbageTypeVisitor;

impl<'de> Visitor<'de> for GarbageTypeVisitor {
	type Value = GarbageType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("GarbageType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"attack" => Ok(GarbageType::Attack),
			"kev" => Ok(GarbageType::Interaction),
			"interaction" => Ok(GarbageType::Kev),
			"interaction_confirm" => Ok(GarbageType::InteractionConfirm),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for GarbageType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(GarbageTypeVisitor)
	}
}
