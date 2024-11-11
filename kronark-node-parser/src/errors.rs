use std::error::Error;
use std::fmt::Display;
use std::string::FromUtf8Error;
#[derive(Debug)]
pub enum NodeParseError {
    EOF(&'static str, u64),
    InvalidFile(u64),
    InvalidVersion(u64),
    UTF8EncodingError(FromUtf8Error, u64),
    FileToLong(u64),
    InvalidSocketType(u64),
    InvalidSocketCombination(u64),
}

impl Display for NodeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeParseError::EOF(eoferror, byte) => {
                writeln!(f, "Early EOF while reading {}. At byte offset {}", eoferror, byte)
            }
            NodeParseError::InvalidFile(byte) => {
                writeln!(f, "File type not recognized (magic number incorrect), At byte offset {}",byte)
            }
            NodeParseError::InvalidVersion(byte) => writeln!(f, "Invalid Version. At byte offset {} ",byte ),
            NodeParseError::UTF8EncodingError(utf, byte) => {
                writeln!(f, "Could not parse, got invalid UTF8 {}. At byte offset {}", utf,byte)
            }
            NodeParseError::FileToLong(byte) => writeln!(f, "Extra data at the end of parsing. At byte offset: {}",byte),
            NodeParseError::InvalidSocketType(byte) => {
                writeln!(f, "Socket type is an invalid number. At byte offset: {}",byte)
            }
            NodeParseError::InvalidSocketCombination(byte) => writeln!(f,"Socket is marked as repetitive and as type `IncomingSwitch`, an illegal combination. At byte offset: {}",byte),
        }
    }
}

impl Error for NodeParseError {}
