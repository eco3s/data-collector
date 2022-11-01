use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer};

pub fn parse_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	D: Deserializer<'de>,
	T: FromStr + Deserialize<'de>,
	<T as FromStr>::Err: Display, {
	String::deserialize(deserializer)?
		.parse::<T>()
		.map_err(de::Error::custom)
}

#[allow(dead_code)]
pub fn empty_string_to_none<'de, D>(
	deserializer: D,
) -> Result<Option<String>, D::Error>
where D: Deserializer<'de> {
	let s = String::deserialize(deserializer)?;

	Ok(if s.is_empty() { None } else { Some(s) })
}

#[allow(dead_code)]
pub fn split_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where D: Deserializer<'de> {
	Ok(String::deserialize(deserializer)?
		.split(',')
		.map(str::trim)
		.map(str::to_owned)
		.collect::<Vec<_>>())
}
