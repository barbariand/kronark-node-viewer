use itertools::Itertools;

use crate::errors::NodeParseError;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct Roots {
    pub input_root_x: u16,
    pub input_root_y: u16,
    pub output_root_x: u16,
    pub output_root_y: u16,
    pub output_connections: Vec<(u8, u8)>, // Node, Socket
}
impl Roots {
    pub fn parse_table<T>(lexer: &mut Lexer<T>) -> Result<Roots, NodeParseError>
    where
        T: Iterator<Item = u8>,
    {
        // I choose to map to u16s here so that I don't have to cast later when masking and
        // bit-shifting. As I understand it, Rust will not automatically cast primitives so
        // performing bytes[0] << 2 with `Vec<u8>` will cause the upper two bits to be lost.
        let packed_bits: Vec<u16> = lexer.by_ref().take(5).map(|v| v as u16).collect();
        if packed_bits.len() < 5 {
            return Err(NodeParseError::EOF("root positions", lexer.bytes_read()));
        }

        // Separate the bytes out into sets of 10 bits
        let input_root_x = (packed_bits[0] << 2) | (packed_bits[1] >> 6);
        let input_root_y = ((packed_bits[1] & 0b00111111) << 4) | (packed_bits[2] >> 4);
        let output_root_x = ((packed_bits[2] & 0b00001111) << 6) | (packed_bits[3] >> 2);
        let output_root_y = ((packed_bits[3] & 0b00000011) << 8) | packed_bits[4];

        // Parse and construct output connections
        let num_output_connections = lexer.next().ok_or(NodeParseError::EOF(
            "number of output connections",
            lexer.bytes_read(),
        ))?;
        let mut output_connections: Vec<(u8, u8)> = vec![];
        for _ in 0..num_output_connections {
            let (node, socket) = lexer.by_ref().next_tuple().ok_or(NodeParseError::EOF(
                "output connections",
                lexer.bytes_read(),
            ))?;
            output_connections.push((node, socket));
        }

        Ok(Roots {
            input_root_x,
            input_root_y,
            output_root_x,
            output_root_y,
            output_connections,
        })
    }
}
