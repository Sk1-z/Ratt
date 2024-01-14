use com::{color::Color::RESET, printf, printlnf, RUST};
use libc::winsize;
use std::io::*;

enum Box {
    Horizontal = 0x2501,
    Vertical = 0x2503,
    TopLeft = 0x250F,
    TopRight = 0x2513,
    BottomLeft = 0x2517,
    BottomRight = 0x251B,
}

impl Box {
    fn as_char(self) -> char {
        char::from_u32(self as u32).unwrap()
    }
}

use Box::*;

pub struct Display {
    winsz: winsize,
    offset: u16,
}

impl Display {
    pub fn new(winsz: winsize) -> Display {
        printf!("\x1b[?47h\x1b[2J\x1b[H");
        Display { offset: 0, winsz }
    }

    pub fn header(&mut self) {
        printf!(
            "\x1b[2;3f{}{}{} RATT! ",
            RUST,
            TopLeft.as_char(),
            Horizontal.as_char()
        );

        for _ in 0..(self.winsz.ws_col - 14) {
            printf!("{}", Horizontal.as_char());
        }
        printf!("{}", TopRight.as_char());

        for _ in 0..(self.winsz.ws_row - 12) {
            printf!("\x1b[1B\x1b[1D{}", Vertical.as_char());
        }
        printf!("\x1b[1B\x1b[1D{}", BottomRight.as_char());

        for _ in 0..(self.winsz.ws_col - 6) {
            printf!("\x1b[2D{}", Horizontal.as_char());
        }
        printf!("\x1b[2D{}", BottomLeft.as_char());

        for _ in 0..(self.winsz.ws_row - 12) {
            printf!("\x1b[1A\x1b[1D{}", Vertical.as_char());
        }
    }

    pub fn input(&self) {
        printf!("\x1b[{};3f{}", self.winsz.ws_row - 6, TopLeft.as_char());

        for _ in 0..(self.winsz.ws_col - 6) {
            printf!("{}", Horizontal.as_char());
        }
        printf!("{}", TopRight.as_char());

        for _ in 0..3 {
            printf!("\x1b[1B\x1b[1D{}", Vertical.as_char());
        }
        printf!("\x1b[1B\x1b[1D{}", BottomRight.as_char());

        for _ in 0..(self.winsz.ws_col - 6) {
            printf!("\x1b[2D{}", Horizontal.as_char());
        }
        printf!("\x1b[2D{}", BottomLeft.as_char());

        for _ in 0..3 {
            printf!("\x1b[1A\x1b[1D{}", Vertical.as_char());
        }
        printf!("\x1b[1B{}", RESET.as_string());
    }

    pub fn prompt(&self) {
        printf!(
            "{}\x1b[{};3f\x1b[0K{}\x1b[{}C{}\r\x1b[3C-> {}",
            RUST,
            self.winsz.ws_row - 4,
            Vertical.as_char(),
            self.winsz.ws_col - 6,
            Vertical.as_char(),
            RESET.as_string()
        );
    }

    pub fn message(&mut self, msg: &String) {
        if self.offset == (self.winsz.ws_row - 12) {
            self.offset = 0;
            printf!("\x1b[s\x1b[{};0f\x1b[1J", self.winsz.ws_row - 8);
            self.header();
            printf!("\x1b[u");
        }
        printf!("\x1b[s\x1b[{};4f\x1b[0C{}\x1b[u", self.offset + 3, msg);
        self.offset += 1;
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        printlnf!("\x1b[?47l\x1b[0m");
    }
}

impl Clone for Display {
    fn clone(&self) -> Display {
        Display {
            winsz: self.winsz,
            offset: self.offset,
        }
    }
}
