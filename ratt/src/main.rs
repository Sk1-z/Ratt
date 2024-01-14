// use chrono::Local;
// Local::now().format("%H:%M")

use com::{color::Color::*, *};
use std::io::*;
use std::net::{Shutdown, TcpStream};

fn main() {
    let username: String = {
        let mut str = String::new();
        printf!("{}Username -> ", BLUE.as_string());
        stdin().read_line(&mut str).unwrap();
        str.trim().to_string()
    };
    // let username = String::from("Root");

    // let addr: String = {
    //     let mut str = String::new();
    //     printf!("Server address[ipv4:port] ->  ");
    //     stdin().read_line(&mut str).unwrap();
    //     str.trim().to_string()
    // };
    let addr = String::from("0.0.0.0:1024");

    let client = TcpStream::connect(addr);
    match client {
        Ok(client) => {
            let mut reader = BufReader::new(&client);
            let mut writer = BufWriter::new(&client);
            let mut buf = String::new();

            writer.write(format!("{}\n", username).as_bytes()).unwrap();
            writer.flush().unwrap();

            reader.read_line(&mut buf).unwrap();

            if buf != format!("{} Room filled. Shutting down..\n", ERR) {
                printlnf!(
                    "{}{} Connected to {} on {}{}{}",
                    CYAN.as_string(),
                    INFO,
                    buf.trim(),
                    CYAN.as_string(),
                    client.peer_addr().unwrap(),
                    RESET.as_string(),
                );

                loop {
                    buf.clear();
                    printf!("{}->{} ", GREEN.as_string(), RESET.as_string());
                    stdin().read_line(&mut buf).unwrap();

                    if buf.as_bytes()[0] == b'!' {
                        match buf.as_bytes()[1] {
                            b'q' => {
                                writer.write(b"!q\n").unwrap();
                                writer.flush().unwrap();
                                client.shutdown(Shutdown::Both).unwrap();
                                printlnf!("{} Shutting down..", INFO);
                                break;
                            }
                            _ => {
                                printlnf!("{} Invalid command, Use !h to view all.", ERR)
                            }
                        }
                    } else {
                        writer.write(buf.as_bytes()).unwrap();
                        writer.flush().unwrap();
                        buf.clear();
                        reader.read_line(&mut buf).unwrap();
                        printf!("{}", buf);
                    }
                }
            } else {
                printlnf!("{}", buf);
            }
        }
        Err(err) => {
            printlnf!("{} {}", ERR, err)
        }
    }
}
