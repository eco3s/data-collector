use std::{error::Error, fs, path::PathBuf};

use data_collector::downloader::{item::get_items, list::get_list};

fn main() -> Result<(), Box<dyn Error>> {
	let path = PathBuf::from("./downloads/");
	fs::create_dir_all(path.as_path())?;

	let list = get_list()?;
	let items = get_items(list)?;

	for (i, item) in items.iter().enumerate() {
		let mut file_path = path.clone();
		file_path.push(format!("{i}.json"));
		fs::write(file_path, item)?;
	}

	Ok(())
}
