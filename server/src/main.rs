const INFO: &'static str = "\x1b[1;36m[INFO]\x1b[0m";
const ERR: &'static str = "\x1b[1;31m[ERROR]\x1b[0m";
// const SERVER: &'static str = "\x1b[38;2;160;35;50m[SERVER]\x1b[0m";
// const CLIENT: &'static str = "\x1b[1;93m[YOU]\x1b[0m";

mod server;
mod user;

use std::io::*;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use server::Server;
use user::User;

#[macro_export]
macro_rules! printf {
    ($($fmt:tt)*) => {{
        print!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

#[macro_export]
macro_rules! printlnf {
    ($($fmt:tt)*) => {{
        println!($($fmt)*);
        stdout().flush().unwrap();
    }};
}

fn main() {
    // If the server fails to start
    let server = Server::new_dbg().unwrap_or_else(|| Server::local());
    printlnf!(
        "{} Server {} started on {}",
        INFO,
        server.name,
        server.conn.local_addr().unwrap(),
    );

    // Mutex to access the vec on all threads, arc because borrow checker can smd
    let room: Arc<Mutex<Vec<Option<User>>>> = Arc::new(Mutex::new(Vec::new()));
    let conn_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    for client in server.conn.incoming() {
        match client {
            Ok(client) => {
                if server.full(*conn_count.lock().unwrap()) {
                    printlnf!("{} Server cannot accept anymore connections", ERR);
                    continue;
                }

                let mut reader = BufReader::new(&client);
                let mut writer = BufWriter::new(&client);
                let mut buf = String::new();

                reader.read_line(&mut buf).unwrap();
                *conn_count.lock().unwrap() += 1;

                printlnf!(
                    "{} {} Joined. Users Active: {}",
                    INFO,
                    buf.trim().to_string(),
                    *conn_count.lock().unwrap()
                );

                let user = User::new(
                    buf.trim().to_string(),
                    // Client clone is not needed for production. After initialization user gets moved
                    // to thread.
                    client.try_clone().unwrap(),
                    room.lock().unwrap().len(),
                );

                room.lock().unwrap().push(Some(user));

                // This should go inside thread.
                writer
                    .write(format!("Welcome {}!\n", buf.trim().to_string()).as_bytes())
                    .unwrap();

                break;
            }
            Err(err) => {
                printlnf!("{} {}", ERR, err);
            }
        }
    }
}
