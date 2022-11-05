mod config;
mod fetch;
mod fs_utils;
mod r#static;

use std::{
	collections::HashMap, error::Error, fs, num::ParseIntError, path::PathBuf,
};

use data_collector::{
	downloader::{Cached, FetchJson, RawJsonData},
	schema::{species::SerializableType, Id},
	utils::iterators::KeyAndResult,
};

use crate::{
	config::Config,
	fetch::{FetchItem, FetchList, ItemData},
	fs_utils::write_or_retry,
	r#static::{
		DOWNLOAD_PATH, EXPORT_PATH, LIST_INDEX, LIST_PATH, TOTAL_PAGES,
	},
};

fn fetch_all(
	config: &Config,
) -> Result<HashMap<Id, <ItemData as RawJsonData>::Output>, Box<dyn Error>> {
	let cached_list_data = FetchList::new(TOTAL_PAGES).into_data()?;

	if config.cache {
		write_or_retry(&*LIST_PATH, cached_list_data.as_ref())?;
	}

	let cached_list: Vec<Id> = cached_list_data.parse()?.into();

	let cached_items = fs::read_dir(*DOWNLOAD_PATH)?
		.into_iter()
		.map(|e| e.unwrap())
		.filter(|e| e.file_type().unwrap().is_file())
		.map(|e| {
			e.path()
				.file_stem()
				.unwrap()
				.to_str()
				.unwrap()
				.to_owned()
		})
		.filter(|f| f != LIST_INDEX)
		.map(|s| s.parse::<Id>())
		.collect::<Result<Vec<_>, ParseIntError>>()?;

	let items_data: HashMap<Id, ItemData> = cached_list
		.iter()
		.map(|&i| {
			KeyAndResult((
				i,
				FetchItem::new(i, cached_items.contains(&i)).into_data(),
			))
		})
		.collect::<Result<HashMap<Id, ItemData>, <FetchItem as FetchJson>::Error>>(
		)?;

	if config.cache {
		for (index, data) in items_data.iter() {
			if config.cache {
				write_or_retry(
					DOWNLOAD_PATH.join(PathBuf::from(*index)),
					data.as_ref(),
				)?;
			}
		}
	}

	Ok(items_data
		.iter()
		.map(|(&i, d)| KeyAndResult((i, d.parse())))
		.collect::<Result<_, _>>()?)
}

fn main() -> Result<(), Box<dyn Error>> {
	let config = Config {
		cache: true,
		file_type: SerializableType::Json,
	};

	let map = fetch_all(&config)?;

	map.into_iter().try_for_each(
		|(index, species)| -> Result<(), Box<dyn Error>> {
			Ok(write_or_retry(
				EXPORT_PATH.join(format!(
					"{index}.{ext}",
					ext = config.file_type.as_ref()
				)),
				species.serialize_into(&config.file_type)?,
			)?)
		},
	)?;

	Ok(())
}
