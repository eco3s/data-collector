use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

type N = u32;

const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02002v.do";

trait FetchBulk {
	type Output: Send + Sync;
	type Url: Send + Sync;

	fn get_urls(&self) -> &Vec<Self::Url>;
	fn new(urls: Vec<Self::Url>) -> Self;
	fn fetch(url: &Self::Url) -> Self::Output;
	fn fetch_all(&self) -> Vec<Self::Output> {
		self.get_urls()
			.par_iter()
			.map(Self::fetch)
			.collect::<Vec<Self::Output>>()
	}
}

struct FetchBulkImpl {
	urls: Vec<N>,
}

impl FetchBulk for FetchBulkImpl {
	type Output = Result<String, ureq::Error>;
	type Url = N;

	fn get_urls(&self) -> &Vec<Self::Url> { &self.urls }

	fn new(urls: Vec<Self::Url>) -> Self { Self { urls } }

	fn fetch(url: &Self::Url) -> Self::Output {
		Ok(ureq::post(BASEURL)
			.set("Accept", "application/json")
			.send_form(&[
				("clsSno", &url.to_string()),
				("searchClsGbn", "eco"),
			])?
			.into_string()?)
	}
}

pub fn get_items(list: Vec<N>) -> Result<Vec<String>, Box<ureq::Error>> {
	list.iter()
		.map(FetchBulkImpl::fetch)
		.map(|v| v.map_err(Box::new))
		.collect::<Result<Vec<_>, _>>()
}

pub fn get_items_parallel(
	list: Vec<N>,
) -> Result<Vec<String>, Box<ureq::Error>> {
	FetchBulkImpl::new(list)
		.fetch_all()
		.into_iter()
		.map(|v| v.map_err(Box::new))
		.collect::<Result<Vec<_>, _>>()
}
