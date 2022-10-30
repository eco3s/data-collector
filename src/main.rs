use std::{error::Error, fmt::Display, fs, path::PathBuf, str::FromStr};

use serde::{Deserialize, Deserializer};

const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02001l.do";

type N = u32;

#[derive(Deserialize, Debug)]
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

pub fn string_to_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr + Deserialize<'de>,
	<T as FromStr>::Err: Display, {
	String::deserialize(deserializer)?
		.parse::<T>()
		.map_err(serde::de::Error::custom)
}

fn main() -> Result<(), Box<dyn Error>> {
	let path = PathBuf::from("./downloads/");
	fs::create_dir_all(path.as_path())?;

	let list: Vec<N> = ureq::post(BASEURL)
		.set("Accept", "application/json")
		.send_form(&[
			("searchYn", "Y"),
			("searchClsGbn", "eco"),
			("pageUnit", "35"),
		])?
		.into_json::<List>()?
		.into();

	for item in list.iter().take(1) {
		println!("{item}");

		let item_data = ureq::post(BASEURL)
			.set("Accept", "application/json")
			.send_form(&[
				("clsSno", &item.to_string()),
				("searchClsGbn", "eco"),
			])?
			.into_string()?;

		let mut file_path = path.clone();
		file_path.push(format!("{item}.json"));
		fs::write(file_path, item_data)?;
	}

	Ok(())
}
