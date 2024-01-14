mod handle;
mod server;
mod user;

use com::*;
use server::Server;
use std::io::*;
use std::net::Shutdown;
use std::sync::{Arc, Mutex};
use std::thread;
use user::User;

fn main() {
    // If the server fails to start
    let server = Server::new_dbg().unwrap_or_else(|| Server::local());
    printlnf!(
        "{} Server {} started on {}",
        INFO,
        server.name,
        server.conn.local_addr().unwrap(),
    );

    let mut thread_pool: Vec<thread::JoinHandle<()>> = Vec::new();
    // Mutex to access the vec on all threads, arc because borrow checker can smd
    let room: Arc<Mutex<Vec<Option<User>>>> = Arc::new(Mutex::new(Vec::new()));
    let conn_count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));

    for client in server.conn.incoming() {
        match client {
            Ok(client) => {
                if server.full(*conn_count.lock().unwrap()) {
                    printlnf!("{} Server cannot accept anymore connections", ERR);
                    BufWriter::new(&client)
                        .write(format!("{} Room filled. Shutting down..\n", ERR).as_bytes())
                        .unwrap();
                    client.shutdown(Shutdown::Both).unwrap();
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
                    buf.trim(),
                    *conn_count.lock().unwrap()
                );

                for client in room.lock().unwrap().iter() {
                    if let Some(client) = client {
                        let mut w = BufWriter::new(&client.conn);
                        w.write(format!("{} {} has joined!\n", SERVER, buf.trim()).as_bytes())
                            .unwrap();
                        w.flush().unwrap();
                    }
                }

                let user = User::new(
                    buf.trim().to_string(),
                    client.try_clone().unwrap(),
                    room.lock().unwrap().len(),
                );

                room.lock().unwrap().push(Some(user.clone()));

                writer
                    .write(format!("{}\n", server.name).as_bytes())
                    .unwrap();
                writer.flush().unwrap();

                let room_clone = Arc::clone(&room);
                let conn_count_clone = Arc::clone(&conn_count);

                thread_pool.push(
                    thread::Builder::new()
                        .name(buf.trim().to_string())
                        .spawn(move || {
                            handle::handle(user, room_clone, conn_count_clone);
                        })
                        .unwrap(),
                )
            }
            Err(err) => {
                printlnf!("{} {}", ERR, err);
            }
        }
    }

    for thread in thread_pool {
        thread.join().unwrap();
    }
}
