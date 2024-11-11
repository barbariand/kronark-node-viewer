use std::ops::Deref;

use crate::errors::NodeParseError;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct NodeEntry(String);
impl Deref for NodeEntry {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl NodeEntry {
	pub fn parse_table<T>(lexer: &mut Lexer<T>) -> Result<Vec<Self>, NodeParseError>
	where
		T: Iterator<Item = u8>,
	{
		let mut nodes = vec![];

		let num_nodes = lexer.next().ok_or(NodeParseError::EOF(
			"number of node strings",
			lexer.bytes_read(),
		))?;
		for _ in 0..num_nodes {
			nodes.push(NodeEntry::from_bytes(lexer)?);
		}

		Ok(nodes)
	}

	pub fn from_bytes<T>(lexer: &mut Lexer<T>) -> Result<NodeEntry, NodeParseError>
	where
		T: Iterator<Item = u8>,
	{
		let name_len = lexer.next().ok_or(NodeParseError::EOF(
			"node string length",
			lexer.bytes_read(),
		))? as usize;
		let name_utf8: Vec<u8> = lexer.by_ref().take(name_len).collect();
		if name_utf8.len() < name_len {
			return Err(NodeParseError::EOF("node string", lexer.bytes_read()));
		}

		match String::from_utf8(name_utf8) {
			Ok(v) => Ok(NodeEntry(v)),
			Err(e) => Err(NodeParseError::UTF8EncodingError(e, lexer.bytes_read())),
		}
	}
}
