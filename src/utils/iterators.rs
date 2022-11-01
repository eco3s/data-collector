use std::{collections::HashMap, hash::Hash};

pub struct KeyAndResult<K, V, E>(pub (K, Result<V, E>))
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
	E: Send + Sync;

impl<K, V, E> FromIterator<KeyAndResult<K, V, E>> for Result<HashMap<K, V>, E>
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
	E: Send + Sync,
{
	fn from_iter<I: IntoIterator<Item = KeyAndResult<K, V, E>>>(
		iter: I,
	) -> Self {
		let iter = iter.into_iter();

		// TODO: use HashMap::with_capacity() instead if possible
		let mut h = HashMap::new();

		for KeyAndResult((k, re)) in iter {
			match re {
				Ok(v) => {
					let _ = h.insert(k, v);
				},
				Err(e) => return Err(e),
			}
		}

		Ok(h)
	}
}
