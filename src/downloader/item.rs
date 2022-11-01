use std::{collections::HashMap, error::Error, hash::Hash};

use crate::utils::{iterators::KeyAndResult, macros::swap};

pub trait FetchBulk {
	type Output: Send + Sync + Eq + Hash;
	type Url: Send + Sync + Eq + Hash;
	type Error: Error + Send + Sync;

	fn get_urls(&self) -> &Vec<Self::Url>;
	fn into_urls(self) -> Vec<Self::Url>;
	fn new(urls: Vec<Self::Url>) -> Self;
	fn fetch(url: &Self::Url) -> Result<Self::Output, Self::Error>;
	fn fetch_all(
		self,
	) -> Result<HashMap<Self::Url, Self::Output>, Self::Error>
	where Self: Sized {
		self.into_urls()
			.into_iter()
			.map(|u| KeyAndResult(swap!(u, Self::fetch(&u))))
			.collect()
	}
}
