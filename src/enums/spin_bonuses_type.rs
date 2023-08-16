use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum SpinBonusesType
{
	None,
	Stupid,
	Handheld,
	All,
	TSpins,
}

impl Serialize for SpinBonusesType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("SpinBonusesType", 5)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct SpinBonusesTypeVisitor;

impl<'de> Visitor<'de> for SpinBonusesTypeVisitor {
	type Value = SpinBonusesType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("SpinBonusesType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"all" => Ok(SpinBonusesType::All),
			"T-spins" => Ok(SpinBonusesType::TSpins),
			"handheld" => Ok(SpinBonusesType::Handheld),
			"stupid" => Ok(SpinBonusesType::Stupid),
			"none" => Ok(SpinBonusesType::None),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for SpinBonusesType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(SpinBonusesTypeVisitor)
	}
}
