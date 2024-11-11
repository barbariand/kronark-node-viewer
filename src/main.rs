mod kronarknode;

fn main() {
	let filepath = std::env::args().nth(1).unwrap();
	let data = std::fs::read(filepath).unwrap();

	println!("{:?}", kronarknode::NodeDefinition::from_bytes(data));
}
