use std::ops::Deref;

#[derive(Debug)]
pub struct NodeEntry(String);
impl Deref for NodeEntry {
	type Target = String;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl NodeEntry {
	pub fn parse_table<T>(byte_iterable: T) -> Result<Vec<Self>, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();
		let mut nodes = vec!();

		let num_nodes = iter.next().ok_or("EOF while reading number of node strings")?;
		for _ in 0..num_nodes {
			nodes.push(NodeEntry::from_bytes(iter.by_ref())?);
		}

		Ok(nodes)
	}

	pub fn from_bytes<T>(byte_iterable: T) -> Result<NodeEntry, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();

		let name_len = iter.next().ok_or("EOF while reading node string length")? as usize;
		let name_utf8: Vec<u8> = iter.by_ref().take(name_len).collect();
		if name_utf8.len() < name_len { return Err("EOF while reading node string"); }

		match String::from_utf8(name_utf8) {
			Ok(v) => Ok(NodeEntry(v)),
			Err(_) => return Err("UTF-8 encoding error while reading node string"),
		}
	}
}
