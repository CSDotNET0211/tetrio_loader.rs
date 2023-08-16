use std::fmt::{Error, Formatter};
use serde::de::Visitor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::ser::SerializeStruct;

#[derive(Debug, Clone, PartialEq)]
pub enum KicksetType {
	SRS,
	SRSPlus,
	SRSX,
	TETRAX,
	NRS,
	ARS,
	ASC,
	None,
}

impl Serialize for KicksetType {
	fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
		where
			S: Serializer,
	{
		let mut state = serializer.serialize_struct("KicksetType", 8)?;
		state.serialize_field("type", &self)?;
		state.end()
	}
}


struct KicksetTypeVisitor;

impl<'de> Visitor<'de> for KicksetTypeVisitor {
	type Value = KicksetType;

	fn expecting(&self, formatter: &mut Formatter) -> Result<(), Error> {
		formatter.write_str("KicksetType")
	}

	fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
		where
			E: serde::de::Error,
	{
		match v {
			"NONE" => Ok(KicksetType::None),
			"ARS" => Ok(KicksetType::ARS),
			"NRS" => Ok(KicksetType::NRS),
			"SRS" => Ok(KicksetType::SRS),
			"SRS-X" => Ok(KicksetType::SRSX),
			"SRS+" => Ok(KicksetType::SRSPlus),
			"TETRA-X" => Ok(KicksetType::TETRAX),
			"ASC" => Ok(KicksetType::ASC),
			_ => panic!("unknown type:{}", v),
		}
	}
}

impl<'de> Deserialize<'de> for KicksetType {
	fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
		where
			D: Deserializer<'de>,
	{
		deserializer.deserialize_identifier(KicksetTypeVisitor)
	}
}
