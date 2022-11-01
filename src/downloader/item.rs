use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub trait FetchBulk {
	type Output: Send + Sync;
	type Url: Send + Sync;
	type Error: Send;

	fn get_urls(&self) -> &Vec<Self::Url>;
	fn new(urls: Vec<Self::Url>) -> Self;
	fn fetch(url: &Self::Url) -> Result<Self::Output, Self::Error>;
	fn fetch_all(&self) -> Result<Vec<Self::Output>, Self::Error> {
		self.get_urls()
			.iter()
			.map(Self::fetch)
			.collect::<_>()
	}
	fn fetch_all_parallel(&self) -> Result<Vec<Self::Output>, Self::Error> {
		self.get_urls()
			.par_iter()
			.map(Self::fetch)
			.collect::<_>()
	}
}
