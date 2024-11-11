use std::ops::Deref;

#[derive(Debug)]
pub struct TypeEntry(String);
impl Deref for TypeEntry {
	type Target = String;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl TypeEntry {
	// Currently a complete copy of `Nodes::parse_table` with some variables renamed
	pub fn parse_table<T>(byte_iterable: T) -> Result<Vec<Self>, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();
		let mut types = vec!();

		let num_types = iter.next().ok_or("EOF while reading number of type strings")?;
		for _ in 0..num_types {
			types.push(TypeEntry::from_bytes(iter.by_ref())?);
		}

		Ok(types)
	}

	pub fn from_bytes<T>(byte_iterable: T) -> Result<Self, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();

		let name_len = iter.next().ok_or("EOF while reading type string length")? as usize;
		let name_utf8: Vec<u8> = iter.by_ref().take(name_len).collect();
		if name_utf8.len() < name_len { return Err("EOF while reading type string"); }

		match String::from_utf8(name_utf8) {
			Ok(v) => Ok(TypeEntry(v)),
			Err(_) => return Err("UTF-8 encoding error while reading type string"),
		}
	}
}
