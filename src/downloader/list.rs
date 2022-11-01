use std::{error::Error, fmt::Display, str::FromStr};

use serde::Deserialize;

pub const BASEURL: &str = "https://kias.nie.re.kr/home/for/for02001l.do";

#[derive(Deserialize)]
#[serde(bound = "T: for <'a>Deserialize<'a>")]
struct DeserializeList<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display, {
	#[serde(rename(deserialize = "clsList"))]
	cls_list: Vec<DeserializeItem<T>>,
}

#[derive(Deserialize, Debug)]
#[serde(bound = "T: for <'a>Deserialize<'a>")]
struct DeserializeItem<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display, {
	#[serde(deserialize_with = "crate::utils::deserializers::parse_number")]
	cls_sno: T,
}

#[derive(Deserialize, Debug)]
#[serde(from = "DeserializeList<T>")]
#[serde(bound = "T: for <'a>Deserialize<'a>")]
pub struct List<T>(Vec<T>)
where
	T: FromStr,
	<T as FromStr>::Err: Display;

impl<T> AsRef<Vec<T>> for List<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display,
{
	fn as_ref(&self) -> &Vec<T> { &self.0 }
}

impl<T> From<DeserializeList<T>> for List<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display,
{
	fn from(value: DeserializeList<T>) -> Self {
		Self(
			value
				.cls_list
				.into_iter()
				.map(|n| n.cls_sno)
				.collect::<Vec<_>>(),
		)
	}
}

impl<T> From<List<T>> for Vec<T>
where
	T: FromStr,
	<T as FromStr>::Err: Display,
{
	fn from(value: List<T>) -> Self { value.0 }
}

pub trait JsonFetcher {
	type Output: for<'de> serde::Deserialize<'de>;
	type FetchError: Error + 'static;

	fn fetch(&self) -> Result<String, Self::FetchError>;

	fn parse(&self, raw: &str) -> serde_json::Result<Self::Output> {
		serde_json::from_str::<Self::Output>(raw)
	}

	fn parse_into(&self, raw: String) -> serde_json::Result<Self::Output> {
		self.parse(&raw)
	}

	fn fetch_and_parse(&self) -> Result<Self::Output, Box<dyn Error>> {
		Ok(self.parse_into(self.fetch()?)?)
	}
}
