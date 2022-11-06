use lib::schema::species::SerializableType;

pub struct Config {
	pub cache: bool,
	pub file_type: SerializableType,
}
