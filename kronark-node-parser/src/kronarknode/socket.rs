use itertools::Itertools;

use crate::errors::NodeParseError;
use crate::lexer::Lexer;

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
            6..=7 => panic!("invalid socket type"),
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
    pub fn from_byte(byte: u8, byte_read: u64) -> Result<SocketFlags, NodeParseError> {
        let socket_type = (byte & 0b00111000) >> 3;
        if socket_type > 5 {
            return Err(NodeParseError::InvalidSocketType(byte_read));
        }

        let flags = SocketFlags(byte);
        if flags.is_repetitive() && matches!(flags.get_type(), SocketType::IncomingSwitch) {
            return Err(NodeParseError::InvalidSocketCombination(byte_read));
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
    pub fn from_bytes<T>(lexer: &mut Lexer<T>) -> Result<Self, NodeParseError>
    where
        T: Iterator<Item = u8>,
    {
        let flags = SocketFlags::from_byte(
            lexer
                .next()
                .ok_or(NodeParseError::EOF("socket flags", lexer.bytes_read()))?,
            lexer.bytes_read(),
        )?;
        let type_index = lexer
            .next()
            .ok_or(NodeParseError::EOF("socket type index", lexer.bytes_read()))?;
        let port_slot = lexer
            .next()
            .ok_or(NodeParseError::EOF("socket port slot", lexer.bytes_read()))?;

        let mut data = None;
        if flags.get_type().is_incoming() {
            if flags.is_connected() {
                let (node, socket) = lexer
                    .next_tuple()
                    .ok_or(NodeParseError::EOF("socket connection", lexer.bytes_read()))?;
                data = Some(DataType::Connection(node, socket));
            } else if !matches!(flags.get_type(), SocketType::IncomingSwitch) {
                // TODO: this is a dirty fix, I want to move this to something cleaner
                let value_len_bytes: [u8; 4] =
                    match lexer.by_ref().take(4).collect::<Vec<u8>>().try_into() {
                        Ok(v) => v,
                        Err(_) => {
                            return Err(NodeParseError::EOF(
                                "socket constant value length",
                                lexer.bytes_read(),
                            ))
                        }
                    };
                let value_len = u32::from_be_bytes(value_len_bytes) as usize;
                let value_bytes: Vec<u8> = lexer.by_ref().take(value_len).collect();
                if value_bytes.len() < value_len {
                    return Err(NodeParseError::EOF(
                        "socket constant value",
                        lexer.bytes_read(),
                    ));
                }
                let value: String = match String::from_utf8(value_bytes) {
                    Ok(v) => v,
                    Err(e) => return Err(NodeParseError::UTF8EncodingError(e, lexer.bytes_read())),
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
