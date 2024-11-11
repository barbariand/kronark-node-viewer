/// Idk if this is to mutch abastraction over it.
pub struct Lexer<I: Iterator<Item = u8>> {
	///For error handeling so we know where it failed :)
	read_bytes: u64,
	bytes: I,
}
impl<I: Iterator<Item = u8>> Lexer<I> {
	pub fn new(iter: I) -> Self {
		Self {
			read_bytes: 0,
			bytes: iter,
		}
	}
	pub fn bytes_read(&self) -> u64 {
		self.read_bytes
	}
}
impl<I: Iterator<Item = u8>> Iterator for Lexer<I> {
	type Item = u8;

	fn next(&mut self) -> Option<Self::Item> {
		let b = self.bytes.next()?;
		self.read_bytes += 1;
		Some(b)
	}
}
