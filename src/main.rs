mod r#static;

use std::{error::Error, fs, path::PathBuf};

use data_collector::{
	downloader::{
		item::FetchBulk,
		list::{JsonFetcher, List},
	},
	parse::parse_item,
};
use r#static::{ITEM_ENDPOINT, LIST_ENDPOINT};

const DOWNLOAD_PATH: &str = "./downloads/";
const EXPORT_PATH: &str = "./out/";

struct ListFetcher(usize);

impl JsonFetcher for ListFetcher {
	type FetchError = ureq::Error;
	type Output = List<u32>;

	fn fetch(&self) -> Result<String, Self::FetchError> {
		Ok(
			ureq::request_url("POST", &LIST_ENDPOINT)
				.set("Accept", "application/json")
				.send_form(&[
					("searchYn", "Y"),
					("searchClsGbn", "eco"),
					("pageUnit", &self.0.to_string()),
				])?
				.into_string()?,
		)
	}
}

struct FetchBulkItem {
	urls: Vec<u32>,
}

impl FetchBulk for FetchBulkItem {
	type Error = ureq::Error;
	type Output = String;
	type Url = u32;

	fn get_urls(&self) -> &Vec<Self::Url> { &self.urls }

	fn new(urls: Vec<Self::Url>) -> Self { Self { urls } }

	fn fetch(url: &Self::Url) -> Result<Self::Output, Self::Error> {
		Ok(
			ureq::request_url("POST", &ITEM_ENDPOINT)
				.set("Accept", "application/json")
				.send_form(&[
					("clsSno", &url.to_string()),
					("searchClsGbn", "eco"),
				])?
				.into_string()?,
		)
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let path = PathBuf::from(DOWNLOAD_PATH);
	let export_path = PathBuf::from(EXPORT_PATH);
	fs::create_dir_all(path.as_path())?;
	fs::create_dir_all(export_path.as_path())?;

	let list_fetcher = ListFetcher(35);
	let list = list_fetcher.fetch_and_parse()?.into();

	let fetch_bulk_item = FetchBulkItem::new(list);
	let items = fetch_bulk_item.fetch_all()?;

	for (i, item) in items.iter().enumerate() {
		let data = parse_item(item)?;

		let mut file_path = path.clone();
		file_path.push(format!("{i}.json"));
		fs::write(file_path, item)?;

		let mut file_path = export_path.clone();
		let file_type = data_collector::schema::SerializableType::Json;

		let ext = match file_type {
			data_collector::schema::SerializableType::Json => "json",
			data_collector::schema::SerializableType::Yaml => "yaml",
			data_collector::schema::SerializableType::Ron => "ron",
			data_collector::schema::SerializableType::Sexpr => "lisp",
		};

		file_path.push(format!("{i}.{ext}"));
		fs::write(
			file_path,
			data.serialize_into(file_type)?,
		)?;
	}

	Ok(())
}
