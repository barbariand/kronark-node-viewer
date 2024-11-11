use std::cmp::Ordering;

mod instance;
mod nodes;
mod roots;
mod socket;
mod types;

use instance::Instance;
use nodes::NodeEntry;
use roots::Roots;
use types::TypeEntry;

#[derive(Debug)]
pub struct NodeDefinition {
	pub roots: Roots,
	pub nodes: Vec<NodeEntry>,
	pub types: Vec<TypeEntry>,
	pub instances: Vec<Instance>,
}
impl NodeDefinition {
	pub fn from_bytes<T>(byte_iterable: T) -> Result<Self, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		// I'm not a huge fan of passing around this iterator like this, but it really does
		// seem like the most flexible and natural way to handle the bytes that I can think
		// of. Open to suggestions for a better way, because I ended up writing the following
		// line of code *quite* a few times.
		let mut iter = byte_iterable.into_iter();

		// Check header
		let magic_number: Vec<u8> = "kronarknode".into();
		let data_magic_number: Vec<u8> = iter.by_ref().take(magic_number.len()).collect();
		match magic_number.cmp(&data_magic_number) {
			Ordering::Equal => (),
			_ => return Err("File type not recognized (magic number incorrect)"),
		}

		let version_number: u8 = match iter.next() {
			Some(v) => v,
			None => return Err("Early EOF: no version number"),
		};

		// Pass iterator around to parse components
		let def = NodeDefinition {
			roots: Roots::parse_table(iter.by_ref())?,
			nodes: NodeEntry::parse_table(iter.by_ref())?,
			types: TypeEntry::parse_table(iter.by_ref())?,
			instances: Instance::parse_table(iter.by_ref())?,
		};

		// If anything remains, assume parsing went wrong and error
		if iter.count() > 0 { return Err("Extra data at the end of parsing"); }

		Ok(def)
	}
}
