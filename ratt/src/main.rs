mod display;
mod handle;

use com::{color::Color::*, *};
use display::Display;
use handle::Handle;
use libc::{ioctl, winsize, TIOCGWINSZ};
use std::io::*;
use std::net::TcpStream;
use std::os::fd::AsRawFd;

fn main() {
    let username: String = {
        let mut str = String::new();
        printf!("{}Username -> ", BLUE.as_string());
        stdin().read_line(&mut str).unwrap();
        str.trim().to_string()
    };

    let addr: String = {
        let mut str = String::new();
        printf!("Server address[ipv4:port] ->  ");
        stdin().read_line(&mut str).unwrap();
        str.trim().to_string()
    };

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
                let winsz: winsize = winsize {
                    ws_row: 0,
                    ws_col: 0,
                    ws_xpixel: 0,
                    ws_ypixel: 0,
                };

                unsafe {
                    ioctl(stdout().as_raw_fd(), TIOCGWINSZ, &winsz);
                }

                let mut display = Display::new(winsz);

                display.header();
                display.input();
                display.prompt();
                display.message(&format!("{} Use '!q' to quit", INFO));
                display.message(&format!(
                    "{} Connected to {} on {}{}{}",
                    INFO,
                    buf.trim(),
                    CYAN.as_string(),
                    client.peer_addr().unwrap(),
                    RESET.as_string(),
                ));

                let handle = Handle::new(&client);
                handle.handle(&mut display);
            } else {
                printlnf!("{}", buf);
            }
        }
        Err(err) => {
            printlnf!("{} {}", ERR, err)
        }
    }
}
