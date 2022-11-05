pub mod list;
pub mod species;

use std::{fmt::Display, num::ParseIntError, path::PathBuf, str::FromStr};

pub use list::List;
use serde::{Deserialize, Serialize};
pub use species::Species;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Id(pub u16);

impl Display for Id {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

impl From<Id> for PathBuf {
	fn from(value: Id) -> Self { Self::from(format!("{value}.json")) }
}

impl FromStr for Id {
	type Err = ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> { s.parse().map(Self) }
}
