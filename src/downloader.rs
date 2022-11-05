use std::error::Error;

use serde::Deserialize;

pub trait FetchJson {
	type Error: Error;
	type Output: RawJsonData;

	fn fetch(self) -> Result<Self::Output, Self::Error>;
}

pub trait Cached: Sized + FetchJson {
	fn retrieve(&self) -> Option<<Self as FetchJson>::Output>;

	fn into_data(
		self,
	) -> Result<<Self as FetchJson>::Output, <Self as FetchJson>::Error> {
		self.retrieve()
			.map_or_else(|| self.fetch(), Ok)
	}
}

pub trait RawJsonData: AsRef<str> {
	type Output: for<'d> Deserialize<'d>;

	fn parse(&self) -> Result<Self::Output, serde_json::Error> {
		serde_json::from_str(self.as_ref())
	}
}
