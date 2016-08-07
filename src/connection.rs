use mio::tcp::TcpStream;
use mio::Token;
use bytes::{Buf, Take};
use std::io::Cursor;

pub enum State {
    Reading(Vec<u8>),
    Writing(Take<Cursor<Vec<u8>>>),
    Closed,
}

impl State {
    pub fn read_buf(&self) -> &Vec<u8> {
        match self {
            &State::Reading(ref v) => v,
            _ => panic!("try to read from buffer, but state is not read")
        }
    }
}

pub struct Connection {
    pub stream : TcpStream,
    pub token : Token,
    pub state : State
}

impl Connection {
    pub fn new(stream : TcpStream, token : Token) -> Connection {
        Connection {
            stream: stream,
            token: token,
            state: State::Reading(vec![])
        }
    }
}
