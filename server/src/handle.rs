use crate::User;
use chrono::Local;
use com::{color::Color::*, *};
use std::io::*;
use std::sync::{Arc, Mutex};

fn handle_dc(
    dc: bool,
    user: &User,
    room: &Arc<Mutex<Vec<Option<User>>>>,
    conn_count: &Arc<Mutex<usize>>,
) {
    (*room.lock().unwrap())[user.id] = None;
    *conn_count.lock().unwrap() -= 1;

    printlnf!(
        "{} {} Left. Users active: {}",
        INFO,
        user.name,
        *conn_count.lock().unwrap(),
    );

    for client in room.lock().unwrap().iter() {
        if let Some(client) = client {
            let mut w = BufWriter::new(&client.conn);
            w.write(
                format!("{} {} has {}!\n", SERVER, user.name, {
                    if dc {
                        "disconnected"
                    } else {
                        "left"
                    }
                })
                .as_bytes(),
            )
            .unwrap();
            w.flush().unwrap();
        }
    }
}

pub fn handle(user: User, room: Arc<Mutex<Vec<Option<User>>>>, conn_count: Arc<Mutex<usize>>) {
    let mut reader = BufReader::new(&user.conn);
    let mut buf = String::new();

    loop {
        buf.clear();

        let read = reader.read_line(&mut buf);
        match read {
            Ok(sz) => {
                if sz == 0 {
                    handle_dc(true, &user, &room, &conn_count);
                    break;
                }
            }
            Err(_) => {
                handle_dc(true, &user, &room, &conn_count);
                break;
            }
        }

        if buf.len() > 0 && buf.as_bytes()[0] == b'!' {
            match buf.as_bytes()[1] {
                b'q' => {
                    handle_dc(false, &user, &room, &conn_count);
                    break;
                }
                _ => {}
            }
        }

        let msg = format!(
            "{}{}{} {} {}",
            CYAN.as_string(),
            Local::now().format("%H:%M"),
            RESET.as_string(),
            user.tag(),
            buf
        );
        for client in room.lock().unwrap().iter() {
            if let Some(client) = client {
                let mut writer = BufWriter::new(&client.conn);
                writer.write(msg.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }
}
