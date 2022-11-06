use std::{fs, path::PathBuf};

use lib::{
	downloader::{Cached, FetchJson, RawJsonData},
	schema::{Id, List, Species},
};

use crate::r#static::{DOWNLOAD_PATH, ITEM_ENDPOINT, LIST_ENDPOINT, LIST_PATH};

pub struct FetchList {
	take: u32,
}

impl FetchList {
	pub fn new(take: u32) -> Self { Self { take } }
}

impl FetchJson for FetchList {
	type Error = ureq::Error;
	type Output = ListData;

	fn fetch(self) -> Result<Self::Output, Self::Error> {
		Ok(ListData(
			ureq::request_url("POST", &LIST_ENDPOINT)
				.set("Accept", "application/json")
				.send_form(&[
					("searchYn", "Y"),
					("searchClsGbn", "eco"),
					("pageUnit", &self.take.to_string()),
				])?
				.into_string()?,
		))
	}
}

impl Cached for FetchList {
	fn retrieve(&self) -> Option<Self::Output> {
		fs::read_to_string(&*LIST_PATH)
			.ok()
			.map(ListData)
	}
}

pub struct FetchItem {
	id: Id,
	cached: bool,
}

impl FetchItem {
	pub fn new(id: Id, cached: bool) -> Self { Self { id, cached } }
}

impl From<&FetchItem> for PathBuf {
	fn from(value: &FetchItem) -> Self {
		DOWNLOAD_PATH.join(format!("{}.json", value.id))
	}
}

impl FetchJson for FetchItem {
	type Error = ureq::Error;
	type Output = ItemData;

	fn fetch(self) -> Result<Self::Output, Self::Error> {
		Ok(ItemData(
			ureq::request_url("POST", &ITEM_ENDPOINT)
				.set("Accept", "application/json")
				.send_form(&[
					("clsSno", &self.id.to_string()),
					("searchClsGbn", "eco"),
				])?
				.into_string()?,
		))
	}
}

impl Cached for FetchItem {
	fn retrieve(&self) -> Option<Self::Output> {
		match self.cached {
			true =>
				fs::read_to_string(PathBuf::from(self)) // TODO: use conv instead
					.ok()
					.map(ItemData),
			false => None,
		}
	}
}

pub struct ListData(String);

impl AsRef<str> for ListData {
	fn as_ref(&self) -> &str { &self.0 }
}

impl RawJsonData for ListData {
	type Output = List;
}

pub struct ItemData(String);

impl AsRef<str> for ItemData {
	fn as_ref(&self) -> &str { &self.0 }
}

impl RawJsonData for ItemData {
	type Output = Species;
}
