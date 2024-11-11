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
	pub fn parse_table<T>(byte_iterable: T) -> Result<Vec<Self>, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();
		let mut instances = vec!();

		let num_instances = iter.next().ok_or("EOF while reading number of instances")?;
		for _ in 0..num_instances {
			instances.push(Instance::from_bytes(iter.by_ref())?);
		}

		Ok(instances)
	}
	pub fn from_bytes<T>(byte_iterable: T) -> Result<Self, &'static str>
	where
		T: IntoIterator<Item = u8>
	{
		let mut iter = byte_iterable.into_iter();

		let key = iter.next().ok_or("EOF while reading instance key")? as usize;
		let node_type = iter.next().ok_or("EOF while reading instance type")? as usize;

		// See `kronarknode::roots::Roots::parse_table` for rationale regarding `map` here
		let packed_bits: Vec<u16> = iter.by_ref().take(4).map(|v| v as u16).collect();
		if packed_bits.len() < 4 { return Err("EOF while reading instance position, name, and sockets"); }

		let position_x = (packed_bits[0] << 2) | (packed_bits[1] >> 6);
		let position_y = ((packed_bits[1] & 0b00111111) << 4) | (packed_bits[2] >> 4);

		let name_len = (((packed_bits[2] & 0b00001111) << 2) | (packed_bits[3] >> 6)) as usize;
		let socket_count = (packed_bits[3] & 0b00111111) as usize;

		let name_utf8: Vec<u8> = iter.by_ref().take(name_len).collect();
		if name_utf8.len() < name_len { return Err("EOF while reading instance name"); }

		let name = match String::from_utf8(name_utf8) {
			Ok(v) => v,
			Err(_) => return Err("UTF-8 encoding error while parsing instance name"),
		};

		let mut sockets = vec!();
		for _ in 0..socket_count {
			sockets.push(Socket::from_bytes(iter.by_ref())?);
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
