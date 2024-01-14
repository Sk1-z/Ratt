use com::{color::Color::*, *};
use std::io::{stdin, stdout, Write};
use std::net::TcpListener;

pub struct Server {
    pub name: String,
    max_conn: usize,
    pub conn: TcpListener,
}

impl Server {
    pub fn new() -> Option<Server> {
        printf!("{}Server name -> ", BLUE.as_string());
        let mut name: String = String::new();
        stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();

        let mut max: String = String::new();
        printf!("Max number of concurrent connections -> ");
        stdin().read_line(&mut max).unwrap();
        let max_conn = max.trim().parse::<usize>();
        if let Err(err) = max_conn {
            println!("\r{} {}", ERR, err);
            return None;
        }

        let mut addr: String = String::new();
        printf!("Server address[ipv4:port] ->  ");
        stdin().read_line(&mut addr).unwrap();
        let conn = TcpListener::bind(addr.trim());
        if let Err(err) = conn {
            println!("\r{} {}", ERR, err);
            return None;
        }

        Some(Server {
            name,
            max_conn: max_conn.unwrap(),
            conn: conn.unwrap(),
        })
    }

    pub fn local() -> Server {
        Server {
            name: String::from("localhost"),
            max_conn: 1,
            conn: TcpListener::bind("0.0.0.0:1024").unwrap(),
        }
    }

    pub fn new_dbg() -> Option<Server> {
        Some(Server {
            name: String::from("Server"),
            max_conn: 2,
            conn: TcpListener::bind("0.0.0.0:1024").unwrap(),
        })
    }

    pub fn full(&self, connection_count: usize) -> bool {
        self.max_conn == connection_count
    }
}
