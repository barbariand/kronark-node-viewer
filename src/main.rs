use std::io::Read;

use clap::Parser;
use clap_stdin::FileOrStdin;
use kronark_node_parser::prelude::*;
/// Parsing args, maybe we can read a node definition from pipes eventually
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(default_value = "-")]
    filepath: FileOrStdin,
}

fn main() {
    let args = Args::parse();
    let data = args.filepath.into_reader().expect("Failed to read std in");

    let node_res = Node::from_bytes(ShortCircuitedReadIterator::new(data));
    match node_res {
        Ok(node) => println!("{:#?}", node),
        Err(error) => eprintln!("{}", error),
    }
}

struct ShortCircuitedReadIterator<R: Read> {
    read: R,
    done: bool,
}

impl<R: Read> ShortCircuitedReadIterator<R> {
    fn new(read: R) -> Self {
        Self { read, done: false }
    }
}

impl<R: Read> Iterator for ShortCircuitedReadIterator<R> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            true => return None,
            false => {
                let mut buf = [0u8];
                match self.read.read_exact(&mut buf) {
                    Ok(_) => Some(buf[0]),
                    Err(_) => {
                        self.done = true;
                        None
                    }
                }
            }
        }
    }
}
