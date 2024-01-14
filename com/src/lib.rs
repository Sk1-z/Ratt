pub const INFO: &'static str = "\x1b[1;36m[INFO]\x1b[0m";
pub const ERR: &'static str = "\x1b[1;31m[ERROR]\x1b[0m";
pub const SERVER: &'static str = "\x1b[38;2;160;35;50m[SERVER]\x1b[0m";

pub const RUST: &'static str = "\x1b[38;2;250;80;0m";

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

pub mod color;
