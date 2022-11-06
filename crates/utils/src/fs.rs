use std::{
	fs,
	io::{self, ErrorKind},
	path::Path,
};

pub fn write_or_retry<P: AsRef<Path>, C: AsRef<[u8]>>(
	path: P,
	contents: C,
) -> io::Result<()> {
	match fs::write(path.as_ref(), contents.as_ref()) {
		Err(e) if matches!(e.kind(), ErrorKind::NotFound) => {
			match path.as_ref().parent() {
				Some(dir) => {
					fs::create_dir_all(dir)?;

					fs::write(path, contents.as_ref())
				},
				None => Err(ErrorKind::Other.into()),
			}
		},
		v => v,
	}
}
