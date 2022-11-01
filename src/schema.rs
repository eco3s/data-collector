use std::{
	error::Error,
	fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

use crate::downloader::list::N;

#[derive(Serialize)]
pub struct Species {
	pub id: N,
	pub name: LocalName,
	pub scientific_name: String,
	pub group: Group,
	pub taxonomic_tree: TaxonomicTree,
	pub image: String,
	pub ktsn: String,
	pub government_designation: Vec<String>,
	pub docs: Vec<String>,
	pub r#type: Type,
	pub data: Data,
}

pub enum SerializableType {
	Json,
	Yaml,
	Ron,
	Sexpr,
}

impl Species {
	pub fn serialize_into(
		&self,
		r#type: SerializableType,
	) -> Result<String, Box<dyn Error>> {
		Ok(match r#type {
			SerializableType::Json => self.to_string(),
			SerializableType::Yaml => serde_yaml::to_string(self)?,
			SerializableType::Ron => ron::to_string(self)?,
			SerializableType::Sexpr => serde_lexpr::to_string(self)?,
		})
	}
}

impl Display for Species {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			serde_json::to_string(self).or(Err(std::fmt::Error))?,
		)
	}
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
	Animal,
	Plant,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimalAppearance {
	#[serde(rename(deserialize = "li_f_spac"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	description: Option<String>,

	#[serde(rename(deserialize = "li_f_comp"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	comparison: Option<String>,

	#[serde(rename(deserialize = "li_f_sub"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	subspecies: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantFlower {
	#[serde(rename(deserialize = "flw"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	description: Option<String>,

	#[serde(rename(deserialize = "flwcolor"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	color: Option<String>,

	#[serde(rename(deserialize = "flwseq"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	sequence: Option<String>,

	#[serde(rename(deserialize = "flwseq_desc"))]
	#[serde(
		deserialize_with = "crate::utils::deserializers::empty_string_to_none"
	)]
	sequence_description: Option<String>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Data {
	Animal { appearance: AnimalAppearance },
	Plant { flower: PlantFlower },
}

#[derive(Serialize)]
pub struct LocalName {
	pub en: String,
	pub ko: String,
}

#[derive(Serialize)]
pub struct LocalNamePartial {
	pub en: Option<String>,
	pub ko: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum Group {
	#[serde(rename(deserialize = "VP"))]
	Plant,

	#[serde(rename(deserialize = "MM"))]
	Mammalia,

	#[serde(rename(deserialize = "-P"))]
	Pisces,

	#[serde(rename(deserialize = "IN"))]
	Insect,

	#[serde(rename(deserialize = "RP"))]
	Reptile,

	#[serde(rename(deserialize = "IV"))]
	Invertebrate,

	#[serde(rename(deserialize = "AM"))]
	Amphibia,
}

#[derive(Serialize)]
pub struct TaxonomicTree {
	pub phylum: LocalNamePartial,
	pub class: LocalNamePartial,
	pub order: LocalNamePartial,
	pub family: LocalNamePartial,
	pub genus: LocalNamePartial,
}
