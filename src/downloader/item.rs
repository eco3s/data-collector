use std::error::Error;

use super::list::N;

const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02002v.do";

pub fn get_items(list: Vec<N>) -> Result<Vec<String>, Box<dyn Error>> {
	let mut res = vec![];

	for item in list {
		res.push(
			ureq::post(BASEURL)
				.set("Accept", "application/json")
				.send_form(&[
					("clsSno", &item.to_string()),
					("searchClsGbn", "eco"),
				])?
				.into_string()?,
		);
	}

	Ok(res)
}
