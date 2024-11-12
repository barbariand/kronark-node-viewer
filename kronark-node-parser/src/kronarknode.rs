use std::cmp::Ordering;

pub mod instance;
pub mod nodes;
pub mod roots;
pub mod socket;
pub mod types;

use instance::Instance;
use nodes::NodeEntry;
use roots::Roots;
use types::TypeEntry;

use crate::errors::NodeParseError;
use crate::lexer::Lexer;
///A Node definition that has all the relevant info for all versions, to future proof it
#[derive(Debug)]
pub enum Node {
	V1(NodeDefinitionV1),
}
impl Node {
	pub fn from_bytes<T>(byte_iterable: T) -> Result<Self, NodeParseError>
	where
		T: IntoIterator<Item = u8>,
	{
		let mut lexer = Lexer::new(byte_iterable.into_iter());
		// Check header
		let magic_number: Vec<u8> = "kronarknode".into();
		let data_magic_number: Vec<u8> = lexer.by_ref().take(magic_number.len()).collect();
		match magic_number.cmp(&data_magic_number) {
			Ordering::Equal => (),
			_ => return Err(NodeParseError::InvalidFile(lexer.bytes_read())),
		}

		let version_number: u8 = match lexer.next() {
			Some(v) => v,
			None => return Err(NodeParseError::EOF("version number", lexer.bytes_read())),
		};
		match version_number {
			1 => Ok(Node::V1(NodeDefinitionV1::from_bytes(&mut lexer)?)),
			_ => Err(NodeParseError::InvalidVersion(lexer.bytes_read())),
		}
	}
}

#[derive(Debug)]
pub struct NodeDefinitionV1 {
	pub roots: Roots,
	pub nodes: Vec<NodeEntry>,
	pub types: Vec<TypeEntry>,
	pub instances: Vec<Instance>,
}
impl NodeDefinitionV1 {
	pub fn from_bytes<T>(lexer: &mut Lexer<T>) -> Result<Self, NodeParseError>
	where
		T: Iterator<Item = u8>,
	{
		// Pass iterator around to parse components
		let def = NodeDefinitionV1 {
			roots: Roots::parse_table(lexer)?,
			nodes: NodeEntry::parse_table(lexer)?,
			types: TypeEntry::parse_table(lexer)?,
			instances: Instance::parse_table(lexer)?,
		};

		// If anything remains, assume parsing went wrong and error
		if lexer.count() > 0 {
			return Err(NodeParseError::FileTooLong(lexer.bytes_read()));
		}

		Ok(def)
	}
}
