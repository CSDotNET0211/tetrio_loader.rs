use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum BagType {
	Bag7,
	Bag14,
	Pairs,
	Classic,
	TotalMayhem,
}

impl Serialize for BagType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("BagType", 5)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct BagTypeVisitor;

impl<'de> Visitor<'de> for BagTypeVisitor {
	type Value = BagType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("BagType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"7-bag" => Ok(BagType::Bag7),
			"14-bag" => Ok(BagType::Bag14),
			"classic" => Ok(BagType::Classic),
			"pairs" => Ok(BagType::Pairs),
			"total mayhem" => Ok(BagType::TotalMayhem),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for BagType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(BagTypeVisitor)
	}
}
