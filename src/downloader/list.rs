use std::error::Error;

use serde::Deserialize;

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
	#[serde(deserialize_with = "crate::utils::deserializers::parse_number")]
	cls_sno: N,
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
