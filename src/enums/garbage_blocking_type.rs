use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum GarbageBlockingType {
	ComboBlocking,
	LimitedBlocking,
	None,
}

impl Serialize for GarbageBlockingType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("GarbageBlockingType", 3)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct GarbageBlockingTypeVisitor;

impl<'de> Visitor<'de> for GarbageBlockingTypeVisitor {
	type Value = GarbageBlockingType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("GarbageBlockingType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"combo blocking" => Ok(GarbageBlockingType::ComboBlocking),
			"limited blocking" => Ok(GarbageBlockingType::LimitedBlocking),
			"none" => Ok(GarbageBlockingType::None),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for GarbageBlockingType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(GarbageBlockingTypeVisitor)
	}
}
