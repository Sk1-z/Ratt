pub enum Color {
    RESET,
    BLACK = 30,
    RED = 31,
    GREEN = 32,
    YELLOW = 33,
    BLUE = 34,
    MAGENTA = 35,
    CYAN = 36,
    WHITE = 37,
}

impl Color {
    pub fn as_string(self) -> String {
        match self {
            Self::RESET => String::from("\x1b[0m"),
            Self::BLACK => String::from("\x1b[1;30m"),
            Self::RED => String::from("\x1b[1;31m"),
            Self::GREEN => String::from("\x1b[1;32m"),
            Self::YELLOW => String::from("\x1b[1;33m"),
            Self::BLUE => String::from("\x1b[1;34m"),
            Self::MAGENTA => String::from("\x1b[1;35m"),
            Self::CYAN => String::from("\x1b[1;36m"),
            Self::WHITE => String::from("\x1b[1;37m"),
        }
    }
}
