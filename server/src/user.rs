use com::color::Color::*;
use std::net::TcpStream;

pub struct User {
    pub color: u8,
    pub name: String,
    pub conn: TcpStream,
    pub id: usize,
}

impl User {
    pub fn new(name: String, conn: TcpStream, id: usize) -> User {
        User {
            color: rand::random::<u8>(),
            name,
            conn,
            id,
        }
    }

    pub fn tag(&self) -> String {
        format!(
            "\x1b[38;5;{}m[{}]{}",
            self.color,
            self.name,
            RESET.as_string()
        )
    }
}

impl Clone for User {
    fn clone(&self) -> User {
        User {
            color: self.color,
            name: self.name.clone(),
            conn: self.conn.try_clone().unwrap(),
            id: self.id,
        }
    }
}
