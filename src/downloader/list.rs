use std::{error::Error, fmt::Display, str::FromStr};

use serde::Deserialize;

pub const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02001l.do";

#[derive(Deserialize)]
#[serde(bound = "T: for <'a>Deserialize<'a>")]
struct List<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display, {
	#[serde(rename(deserialize = "clsList"))]
	cls_list: Vec<Item<T>>,
}

impl<T> From<List<T>> for Vec<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display,
{
	fn from(value: List<T>) -> Self {
		value
			.cls_list
			.into_iter()
			.map(|n| n.cls_sno)
			.collect::<Vec<_>>()
	}
}

#[derive(Deserialize, Debug)]
#[serde(bound = "T: for <'a>Deserialize<'a>")]
struct Item<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display, {
	#[serde(deserialize_with = "crate::utils::deserializers::parse_number")]
	cls_sno: T,
}

pub fn get_list<T>() -> Result<Vec<T>, Box<dyn Error>>
where
	T: FromStr + for<'a> Deserialize<'a>,
	<T as FromStr>::Err: Display, {
	Ok(ureq::post(BASEURL)
		.set("Accept", "application/json")
		.send_form(&[
			("searchYn", "Y"),
			("searchClsGbn", "eco"),
			("pageUnit", "35"),
		])?
		.into_json::<List<T>>()?
		.into())
}
