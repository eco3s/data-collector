use serde::Deserialize;
use utils::deserializers::parse_number;

use super::Id;

#[derive(Deserialize)]
struct DeserializeList {
	#[serde(rename(deserialize = "clsList"))]
	cls_list: Vec<DeserializeItem>,
}

#[derive(Deserialize, Debug)]
struct DeserializeItem {
	#[serde(deserialize_with = "parse_number")]
	cls_sno: Id,
}

#[derive(Deserialize, Debug)]
#[serde(from = "DeserializeList")]
pub struct List(Vec<Id>);

impl AsRef<Vec<Id>> for List {
	fn as_ref(&self) -> &Vec<Id> { &self.0 }
}

impl From<DeserializeList> for List {
	fn from(value: DeserializeList) -> Self {
		Self(
			value
				.cls_list
				.into_iter()
				.map(|n| n.cls_sno)
				.collect::<Vec<_>>(),
		)
	}
}

impl From<List> for Vec<Id> {
	fn from(value: List) -> Self { value.0 }
}
