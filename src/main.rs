use std::{error::Error, fs, path::PathBuf};

use data_collector::{
	downloader::{item::get_items, list::get_list},
	parse::parse_item,
};

const DOWNLOAD_PATH: &str = "./downloads/";
const EXPORT_PATH: &str = "./out/";

fn main() -> Result<(), Box<dyn Error>> {
	let path = PathBuf::from(DOWNLOAD_PATH);
	let export_path = PathBuf::from(EXPORT_PATH);
	fs::create_dir_all(path.as_path())?;
	fs::create_dir_all(export_path.as_path())?;

	let list = get_list()?;
	let items = get_items(list)?;

	for (i, item) in items.iter().enumerate() {
		let data = parse_item(item)?;

		let mut file_path = path.clone();
		file_path.push(format!("{i}.json"));
		fs::write(file_path, item)?;

		let mut file_path = export_path.clone();
		file_path.push(format!("{i}.json"));
		fs::write(file_path, serde_json::to_string(&data)?)?;
	}

	Ok(())
}
