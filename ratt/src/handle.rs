use crate::display::Display;
use com::{printf, ERR};
use std::io::*;
use std::net::{Shutdown, TcpStream};
use std::sync::mpsc;
use std::thread;

pub struct Handle {
    client: TcpStream,
}

impl Handle {
    pub fn new(client: &TcpStream) -> Handle {
        Handle {
            client: client.try_clone().unwrap(),
        }
    }

    pub fn handle(self, display: &mut Display) {
        let (tx, rx) = mpsc::channel::<u8>();

        let read_client = self.client.try_clone().unwrap();
        let write_client = self.client.try_clone().unwrap();

        let mut read_display = display.clone();
        let mut write_display = display.clone();

        let reader = thread::spawn(move || {
            handle_read(rx, read_client, &mut read_display);
        });

        let writer = thread::spawn(move || {
            handle_write(tx, write_client, &mut write_display);
        });

        reader.join().unwrap();
        writer.join().unwrap();
    }
}

fn handle_read(rx: mpsc::Receiver<u8>, client: TcpStream, display: &mut Display) {
    let mut reader = BufReader::new(&client);
    let mut buf = String::new();

    loop {
        buf.clear();

        let quit = rx.try_recv();
        match quit {
            Ok(quit) => {
                if quit == 1 {
                    break;
                }
            }
            Err(_) => {}
        }

        reader.read_line(&mut buf).unwrap();
        display.message(&buf);
    }
}

fn handle_write(tx: mpsc::Sender<u8>, client: TcpStream, display: &mut Display) {
    let mut writer = BufWriter::new(&client);
    let mut buf = String::new();

    loop {
        buf.clear();

        stdin().read_line(&mut buf).unwrap();
        display.prompt();

        if buf.as_bytes()[0] == b'!' {
            match buf.as_bytes()[1] {
                b'q' => {
                    writer.write(b"!q\n").unwrap();
                    writer.flush().unwrap();
                    client.shutdown(Shutdown::Both).unwrap();
                    tx.send(1).unwrap();
                    break;
                }
                _ => {
                    printf!("{} Invalid command", ERR);
                    stdin().read_line(&mut buf).unwrap();
                    display.prompt();
                }
            }
        } else {
            writer.write(buf.as_bytes()).unwrap();
            writer.flush().unwrap();
        }
    }
}
