use lazy_static::lazy_static;
use url::Url;

lazy_static! {
	static ref BASEURL: Url =
		Url::parse("https://kias.nie.re.kr/home/for/").unwrap();
	pub static ref LIST_ENDPOINT: Url = BASEURL.join("for02001l.do").unwrap();
	pub static ref ITEM_ENDPOINT: Url = BASEURL.join("for02002v.do").unwrap();
}
