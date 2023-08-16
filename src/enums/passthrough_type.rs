use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum PassthroughType {
	Full,
	Limited,
	Zero,
	Consistent,
}

impl Serialize for PassthroughType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("PassthroughType", 4)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct PassthroughTypeVisitor;

impl<'de> Visitor<'de> for PassthroughTypeVisitor {
	type Value = PassthroughType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("PassthroughType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"consistent" => Ok(PassthroughType::Consistent),
			"limited" => Ok(PassthroughType::Limited),
			"full" => Ok(PassthroughType::Full),
			"zero" => Ok(PassthroughType::Zero),
			"true" => Ok(PassthroughType::Full),
			"false" => Ok(PassthroughType::Limited),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for PassthroughType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(PassthroughTypeVisitor)
	}
}
