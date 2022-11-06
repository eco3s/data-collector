use std::{
	error::Error,
	fmt::{self, Display},
};

use serde::{Deserialize, Serialize};
use utils::{
	deserializers::{empty_string_to_none, parse_number, split_string},
	docs_gen, tree_gen,
};

use super::Id;

#[derive(Serialize, Deserialize)]
#[serde(from = "DeserializeSpecies")]
pub struct Species {
	pub id: Id,
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

impl Display for SerializableType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.as_ref())
	}
}

impl AsRef<str> for SerializableType {
	fn as_ref(&self) -> &str {
		match self {
			SerializableType::Json => "json",
			SerializableType::Yaml => "yaml",
			SerializableType::Ron => "ron",
			SerializableType::Sexpr => "lisp",
		}
	}
}

impl Species {
	pub fn serialize_into(
		&self,
		r#type: &SerializableType,
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

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
	Animal,
	Plant,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Data {
	Animal { appearance: AnimalAppearance },
	Plant { flower: PlantFlower },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AnimalAppearance {
	#[serde(rename(deserialize = "li_f_spac"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	description: Option<String>,

	#[serde(rename(deserialize = "li_f_comp"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	comparison: Option<String>,

	#[serde(rename(deserialize = "li_f_sub"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	subspecies: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantFlower {
	#[serde(rename(deserialize = "flw"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	description: Option<String>,

	#[serde(rename(deserialize = "flwcolor"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	color: Option<String>,

	#[serde(rename(deserialize = "flwseq"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	sequence: Option<String>,

	#[serde(rename(deserialize = "flwseq_desc"))]
	#[serde(deserialize_with = "empty_string_to_none")]
	sequence_description: Option<String>,
}

#[derive(Deserialize, Debug)]
struct DeserializeName {
	#[serde(rename(deserialize = "cls_ename"))]
	en: String,

	#[serde(rename(deserialize = "cls_kname"))]
	ko: String,
}

impl From<DeserializeName> for LocalName {
	fn from(DeserializeName { en, ko }: DeserializeName) -> Self {
		Self { en, ko }
	}
}

tree_gen! {
	@TaxonomicTree[LocalNamePartial]
	(DeserializeTaxonomic) Tree {
	#[serde(deserialize_with = "empty_string_to_none")]
		200 |> Phylum
		300 |> Class
		400 |> Order
		500 |> Family
		600 |> Genus
	}
}

docs_gen! {
	Docs [1 -> 2 -> 3 -> 4 -> 5]
}

#[derive(Deserialize, Debug)]
struct Common {
	#[serde(flatten)]
	name: DeserializeName,

	#[serde(rename(deserialize = "cls_hak_full_nm"))]
	scientific_name: String,

	#[serde(rename(deserialize = "cls_sno"))]
	#[serde(deserialize_with = "parse_number")]
	id: Id,

	#[serde(flatten)]
	taxonomic_tree: DeserializeTaxonomicTree,

	#[serde(flatten)]
	docs: Docs,

	#[serde(rename(deserialize = "comm_group"))]
	group: Group,

	#[serde(rename(deserialize = "daepyo_img_path"))]
	image: String,

	ktsn: String,

	#[serde(rename(deserialize = "national_gbn_nm"))]
	#[serde(deserialize_with = "split_string")]
	government_designation: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cls_gbn")]
#[serde(rename_all(deserialize = "UPPERCASE"))]
enum Class {
	Animal {
		#[serde(flatten)]
		common: Common,

		#[serde(flatten)]
		appearance: AnimalAppearance,
	},
	Plant {
		#[serde(flatten)]
		common: Common,

		#[serde(flatten)]
		flower: PlantFlower,
	},
}

impl Class {
	fn get_type(&self) -> Type {
		match self {
			Self::Animal { .. } => Type::Animal,
			Self::Plant { .. } => Type::Plant,
		}
	}
}

#[derive(Deserialize, Debug)]
pub struct DeserializeSpecies {
	cls: Class,
}

impl From<DeserializeSpecies> for Species {
	fn from(value: DeserializeSpecies) -> Self {
		let r#type = value.cls.get_type();

		let (common, data) = match value.cls {
			Class::Animal {
				common, appearance, ..
			} => (common, Data::Animal { appearance }),
			Class::Plant { common, flower, .. } =>
				(common, Data::Plant { flower }),
		};

		Species {
			id: common.id,
			name: common.name.into(),
			scientific_name: common.scientific_name,
			group: common.group,
			taxonomic_tree: common.taxonomic_tree.into(),
			image: common.image,
			ktsn: common.ktsn,
			government_designation: common.government_designation,
			docs: common.docs.0,
			r#type,
			data,
		}
	}
}
