//VT100 Macros
pub const _VT100_CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";

// KEYBOARD
pub const KEYBOARD_CLOSE_SHORTCUT: char = 'q';

// EDITOR STYLE
pub const EDITOR_NAME: &str = env!("CARGO_PKG_NAME");
pub const EDITOR_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const TERMINATION_MESSAGE: &str = "Goodbye!";
pub const SIDE_EDITOR_CHAR: char = '~';
pub const HEADER_DISPLAY_DIVIDER: u16 = 2;

//TERMINAL
pub const TERMINAL_INIT_POSITION: (u16, u16) = (0, 0);
pub const TERMINAL_NEW_LINE: &str = "\r\n";
