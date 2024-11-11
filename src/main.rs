use std::path::PathBuf;

use clap::Parser;
use kronark_node_parser::prelude::*;
/// Parsing args, maybe we can read a node definition from pipes eventually
#[derive(Parser)]
struct Args {
    filepath: PathBuf,
}
fn main() {
	let args=Args::parse();
    let data = std::fs::read(args.filepath).unwrap();

    println!("{:#?}", Node::from_bytes(data));
}
