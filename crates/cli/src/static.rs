use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use url::Url;

pub const TOTAL_PAGES: u32 = 35;
pub const LIST_INDEX: &str = "_index";
pub const DOWNLOAD_EXT: &str = "json";

lazy_static! {
	static ref BASEURL: Url =
		Url::parse("https://kias.nie.re.kr/home/for/").unwrap();
	pub static ref LIST_ENDPOINT: Url = BASEURL.join("for02001l.do").unwrap();
	pub static ref ITEM_ENDPOINT: Url = BASEURL.join("for02002v.do").unwrap();
	pub static ref DOWNLOAD_PATH: &'static Path = Path::new("./downloads");
	pub static ref EXPORT_PATH: &'static Path = Path::new("./out");
	pub static ref LIST_PATH: PathBuf =
		DOWNLOAD_PATH.join(format!("{LIST_INDEX}.{DOWNLOAD_EXT}"));
}
