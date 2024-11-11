use std::ops::Deref;

use crate::errors::NodeParseError;
use crate::lexer::Lexer;

#[derive(Debug)]
pub struct TypeEntry(String);
impl Deref for TypeEntry {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl TypeEntry {
    // Currently a complete copy of `Nodes::parse_table` with some variables renamed
    pub fn parse_table<T>(lexer: &mut Lexer<T>) -> Result<Vec<Self>, NodeParseError>
    where
        T: Iterator<Item = u8>,
    {
        let mut types = vec![];

        let num_types = lexer
            .next()
            .ok_or(NodeParseError::EOF("reading number of type strings",lexer.bytes_read()))?;
        for _ in 0..num_types {
            types.push(TypeEntry::from_bytes(lexer.by_ref())?);
        }

        Ok(types)
    }

    pub fn from_bytes<T>(lexer: &mut Lexer<T>) -> Result<Self, NodeParseError>
    where
        T: Iterator<Item = u8>,
    {
        let name_len =
            lexer.next()
                .ok_or(NodeParseError::EOF("reading type string length",lexer.bytes_read()))? as usize;
        let name_utf8: Vec<u8> = lexer.take(name_len).collect();
        if name_utf8.len() < name_len {
            return Err(NodeParseError::EOF("type string",lexer.bytes_read()));
        }

        match String::from_utf8(name_utf8) {
            Ok(v) => Ok(TypeEntry(v)),
            Err(e) => Err(NodeParseError::UTF8EncodingError(e,lexer.bytes_read())),
        }
    }
}
