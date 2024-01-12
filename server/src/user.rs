// use rand::prelude::*;
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
