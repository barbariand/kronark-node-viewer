use itertools::Itertools;

#[derive(Debug)]
pub enum SocketType {
	OutgoingNamed,
	IncomingNamed,
	IncomingNumber,
	IncomingSelect,
	IncomingSwitch,
	IncomingText,
}
impl SocketType {
	fn from_flags(flags: &SocketFlags) -> Self {
		match (flags.0 & 0b00111000) >> 3 {
			0 => Self::OutgoingNamed,
			1 => Self::IncomingNamed,
			2 => Self::IncomingNumber,
			3 => Self::IncomingSelect,
			4 => Self::IncomingSwitch,
			5 => Self::IncomingText,
			6..7 => panic!("invalid socket type"),
			_ => unreachable!(),
		}
	}

	pub fn is_incoming(&self) -> bool {
		match self {
			Self::OutgoingNamed => false,
			_ => true,
		}
	}
}

#[derive(Debug)]
pub struct SocketFlags(u8);
impl SocketFlags {
	pub fn from_byte(byte: u8) -> Result<SocketFlags, &'static str> {
		let socket_type = (byte & 0b00111000) >> 3;
		if socket_type > 5 { return Err("Socket type is an invalid number"); }

		let flags = SocketFlags(byte);
		if flags.is_repetitive() && matches!(flags.get_type(), SocketType::IncomingSwitch) {
			return Err("Socket is marked as repetitive and as type `IncomingSwitch`, an illegal combination");
		}

		Ok(flags)
	}

	pub fn get_type(&self) -> SocketType {
		SocketType::from_flags(self)
	}
	pub fn is_repetitive(&self) -> bool {
		self.0 & 0b100 != 0
	}
	pub fn is_connected(&self) -> bool {
		self.0 & 0b010 != 0
	}
	pub fn is_switch_on(&self) -> bool {
		self.0 & 0b001 != 0
	}
}

#[derive(Debug)]
pub enum DataType {
	Connection(u8, u8), // Node, Socket
	Constant(String),
}

#[derive(Debug)]
pub struct Socket {
	pub flags: SocketFlags,
	pub type_index: usize,
	pub port_slot: u8,
	pub data: Option<DataType>,
}
impl Socket {
	pub fn from_bytes<T>(byte_iterable: T) -> Result<Self, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();

		let flags = SocketFlags::from_byte(iter.next().ok_or("EOF while reading socket flags")?)?;
		let type_index = iter.next().ok_or("EOF while reading socket type index")?;
		let port_slot = iter.next().ok_or("EOF while reading socket port slot")?;

		let mut data = None;
		if flags.get_type().is_incoming() {
			if flags.is_connected() {
				let (node, socket) = iter.next_tuple().ok_or("EOF while reading socket connection")?;
				data = Some(DataType::Connection(node, socket));
			} else {
				let value_len_bytes: [u8; 4] = match iter.by_ref().take(4).collect::<Vec<u8>>().try_into() {
					Ok(v) => v,
					Err(_) => return Err("Failed to read 4 bytes for socket constant value length"),
				};
				let value_len = u32::from_be_bytes(value_len_bytes) as usize;
				let value_bytes: Vec<u8> = iter.by_ref().take(value_len).collect();
				if value_bytes.len() < value_len { return Err("EOF while reading socket constant value"); }
				let value: String = match String::from_utf8(value_bytes) {
					Ok(v) => v,
					Err(_) => return Err("UTF-8 encoding error when reading socket constant value"),
				};
				data = Some(DataType::Constant(value));
			}
		}

		Ok(Socket {
			flags,
			type_index: type_index.into(),
			port_slot,
			data,
		})
	}
}
