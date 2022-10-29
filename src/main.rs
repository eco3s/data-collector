use std::{error::Error, fs, path::PathBuf};

use serde::Deserialize;

const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02001l.do";

#[derive(Deserialize)]
struct List {
	#[serde(rename(deserialize = "clsList"))]
	cls_list: Vec<Item>,
}

#[derive(Deserialize)]
struct Item {
	cls_sno: String,
}

fn main() -> Result<(), Box<dyn Error>> {
	let path = PathBuf::from("./downloads/");
	fs::create_dir_all(path.as_path())?;

	let list = ureq::post(BASEURL)
		.set("Accept", "application/json")
		.send_form(&[
			("searchYn", "Y"),
			("searchClsGbn", "eco"),
			("pageUnit", "35"),
		])?
		.into_json::<List>()?;

	for Item { cls_sno } in list.cls_list {
		println!("{cls_sno}");

		let item_data = ureq::post(BASEURL)
			.set("Accept", "application/json")
			.send_form(&[("clsSno", &cls_sno), ("searchClsGbn", "eco")])?
			.into_string()?;

		let mut file_path = path.clone();
		file_path.push(format!("{cls_sno}.json"));
		fs::write(file_path, item_data)?;
	}

	Ok(())
}
