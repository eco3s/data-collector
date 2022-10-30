use std::{error::Error, fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer};

pub const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02001l.do";

pub type N = u32;

#[derive(Deserialize)]
struct List {
	#[serde(rename(deserialize = "clsList"))]
	cls_list: Vec<Item>,
}

impl From<List> for Vec<N> {
	fn from(value: List) -> Self {
		value
			.cls_list
			.iter()
			.map(|n| n.cls_sno.to_owned())
			.collect::<Vec<_>>()
	}
}

#[derive(Deserialize, Debug)]
struct Item {
	#[serde(deserialize_with = "string_to_number")]
	cls_sno: N,
}

fn string_to_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr + Deserialize<'de>,
	<T as FromStr>::Err: Display, {
	String::deserialize(deserializer)?
		.parse::<T>()
		.map_err(serde::de::Error::custom)
}

pub fn get_list() -> Result<Vec<N>, Box<dyn Error>> {
	Ok(ureq::post(BASEURL)
		.set("Accept", "application/json")
		.send_form(&[
			("searchYn", "Y"),
			("searchClsGbn", "eco"),
			("pageUnit", "35"),
		])?
		.into_json::<List>()?
		.into())
}
