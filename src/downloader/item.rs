use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub trait FetchBulk {
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
