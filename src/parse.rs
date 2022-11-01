use std::error::Error;

use serde::Deserialize;

use crate::schema::{self, Group};

#[derive(Deserialize, Debug)]
struct Name {
	#[serde(rename(deserialize = "cls_ename"))]
	en: String,

	#[serde(rename(deserialize = "cls_kname"))]
	ko: String,
}

impl From<Name> for schema::LocalName {
	fn from(Name { en, ko }: Name) -> Self { Self { en, ko } }
}

crate::utils::macros::tree_gen! {
	@schema::TaxonomicTree[schema::LocalNamePartial]
	(Taxonomic) Tree {
	#[serde(deserialize_with = "crate::utils::deserializers::empty_string_to_none")]
		200 |> Phylum
		300 |> Class
		400 |> Order
		500 |> Family
		600 |> Genus
	}
}

crate::utils::macros::docs_gen! {
	Docs [1 -> 2 -> 3 -> 4 -> 5]
}

#[derive(Deserialize, Debug)]
struct Common {
	#[serde(flatten)]
	name: Name,

	#[serde(rename(deserialize = "cls_hak_full_nm"))]
	scientific_name: String,

	#[serde(rename(deserialize = "cls_sno"))]
	#[serde(deserialize_with = "crate::utils::deserializers::parse_number")]
	id: u32,

	#[serde(flatten)]
	taxonomic_tree: TaxonomicTree,

	#[serde(flatten)]
	docs: Docs,

	#[serde(rename(deserialize = "comm_group"))]
	group: Group,

	#[serde(rename(deserialize = "daepyo_img_path"))]
	image: String,

	ktsn: String,

	#[serde(rename(deserialize = "national_gbn_nm"))]
	#[serde(deserialize_with = "crate::utils::deserializers::split_string")]
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
		appearance: schema::AnimalAppearance,
	},
	Plant {
		#[serde(flatten)]
		common: Common,

		#[serde(flatten)]
		flower: schema::PlantFlower,
	},
}

impl Class {
	fn get_type(&self) -> schema::Type {
		match self {
			Self::Animal { .. } => schema::Type::Animal,
			Self::Plant { .. } => schema::Type::Plant,
		}
	}
}

#[derive(Deserialize, Debug)]
struct Species {
	cls: Class,
}

impl From<Species> for schema::Species {
	fn from(value: Species) -> Self {
		let r#type = value.cls.get_type();

		let (common, data) = match value.cls {
			Class::Animal {
				common, appearance, ..
			} => (common, schema::Data::Animal {
				appearance,
			}),
			Class::Plant { common, flower, .. } =>
				(common, schema::Data::Plant { flower }),
		};

		schema::Species {
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

pub fn parse_item(item: &str) -> Result<schema::Species, Box<dyn Error>> {
	Ok(serde_json::from_str::<Species>(item)?.into())
}
