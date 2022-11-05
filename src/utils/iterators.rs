use std::{collections::HashMap, hash::Hash};

pub struct KeyAndResult<K, V, E>(pub (K, Result<V, E>))
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
	E: Send + Sync;

impl<K, V, E> From<(K, Result<V, E>)> for KeyAndResult<K, V, E>
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
	E: Send + Sync,
{
	fn from(value: (K, Result<V, E>)) -> Self { value.into() }
}

impl<K, V, E> FromIterator<KeyAndResult<K, V, E>> for Result<HashMap<K, V>, E>
where
	K: Eq + Hash + Send + Sync,
	V: Send + Sync,
	E: Send + Sync,
{
	fn from_iter<I: IntoIterator<Item = KeyAndResult<K, V, E>>>(
		iter: I,
	) -> Self {
		iter.into_iter().try_fold(
			// TODO: use HashMap::with_capacity() instead if possible
			HashMap::new(),
			|mut prev: HashMap<K, V>, KeyAndResult((k, re))| {
				re.map(|v| {
					prev.insert(k, v);
					prev
				})
			},
		)
	}
}
