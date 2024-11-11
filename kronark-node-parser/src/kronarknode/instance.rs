use crate::errors::NodeParseError;
use crate::lexer::Lexer;

use super::socket::Socket;

#[derive(Debug)]
pub struct Instance {
	// Assumptions: key is a unique ID, type is an index entry into the list of nodes
	pub key: usize,
	pub node_type: usize, // called `type` in the documentation, but that is reserved in Rust and I don't want to use a raw identifier
	pub position_x: u16,
	pub position_y: u16,
	pub name: String,
	pub sockets: Vec<Socket>,
}
impl Instance {
	pub fn parse_table<T>(lexer: &mut Lexer<T>) -> Result<Vec<Self>, NodeParseError>
	where
		T: Iterator<Item = u8>,
	{
		let mut instances = vec![];

		let num_instances = lexer.next().ok_or(NodeParseError::EOF(
			"number of instances",
			lexer.bytes_read(),
		))?;

		for _ in 0..num_instances {
			instances.push(Instance::from_bytes(lexer.by_ref())?);
		}

		Ok(instances)
	}
	pub fn from_bytes<T>(lexer: &mut Lexer<T>) -> Result<Self, NodeParseError>
	where
		T: Iterator<Item = u8>,
	{
		let key = lexer.next().ok_or(NodeParseError::EOF(
			"missing instance key",
			lexer.bytes_read(),
		))? as usize;
		let node_type = lexer
			.next()
			.ok_or(NodeParseError::EOF("instance type", lexer.bytes_read()))?
			as usize;

		// See `kronarknode::roots::Roots::parse_table` for rationale regarding `map` here
		let packed_bits: Vec<u16> = lexer.by_ref().take(4).map(|v| v as u16).collect();
		if packed_bits.len() < 4 {
			return Err(NodeParseError::EOF(
				"instance position, name, and sockets",
				lexer.bytes_read(),
			));
		}

		let position_x = (packed_bits[0] << 2) | (packed_bits[1] >> 6);
		let position_y = ((packed_bits[1] & 0b00111111) << 4) | (packed_bits[2] >> 4);

		let name_len = (((packed_bits[2] & 0b00001111) << 2) | (packed_bits[3] >> 6)) as usize;
		let socket_count = (packed_bits[3] & 0b00111111) as usize;

		let name_utf8: Vec<u8> = lexer.by_ref().take(name_len).collect();
		if name_utf8.len() < name_len {
			return Err(NodeParseError::EOF("instance name", lexer.bytes_read()));
		}

		let name = match String::from_utf8(name_utf8) {
			Ok(v) => v,
			Err(e) => return Err(NodeParseError::UTF8EncodingError(e, lexer.bytes_read())),
		};
		let mut sockets = Vec::with_capacity(socket_count);
		for _ in 0..socket_count {
			sockets.push(Socket::from_bytes(lexer.by_ref())?);
		}

		Ok(Instance {
			key,
			node_type,
			position_x,
			position_y,
			name,
			sockets,
		})
	}
}
