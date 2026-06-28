use std::io::Result;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};

use crate::editor::{
    constants::{
        EDITOR_NAME, EDITOR_VERSION, HEADER_DISPLAY_DIVIDER, KEYBOARD_CLOSE_SHORTCUT,
        SIDE_EDITOR_CHAR, TERMINAL_INIT_POSITION, TERMINAL_NEW_LINE, TERMINATION_MESSAGE,
    },
    terminal::Terminal,
};

#[derive(Debug)]
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn new() -> Editor {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap()
    }

    fn repl(&mut self) -> Result<()> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            // Read and evaluate
            let mut event = read()?;
            self.evaluate_event(&mut event);
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        // Quit if it should not be active
        if self.should_quit == true {
            Terminal::clear_screen()?;
            Terminal::print(&format!("{}", TERMINATION_MESSAGE))?;
            Terminal::flush()?;
        } else {
            Self::draw_rows()?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            //Handle for now  code + modifier
            match code {
                KeyCode::Char(KEYBOARD_CLOSE_SHORTCUT) if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn draw_rows() -> Result<()> {
        // Stdout buffer
        let (columns, rows) = Terminal::size()?;
        let header_row: u16 = rows / HEADER_DISPLAY_DIVIDER;
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(TERMINAL_INIT_POSITION)?;

        // Fill buffer with Chars
        for i in 0..rows {
            Terminal::clear_line()?;
            if i == header_row {
                Self::draw_header_line(columns)?
            } else {
                Self::draw_empty_line()?
            }

            // Print eol if not matched height
            if i + 1 < rows {
                Terminal::print(TERMINAL_NEW_LINE)?;
            }
        }

        // Move home again
        Terminal::move_cursor_to(TERMINAL_INIT_POSITION)?;
        Terminal::show_cursor()?;

        // Flush buffer
        Terminal::flush()?;

        Ok(())
    }

    fn draw_header_line(columns: u16) -> Result<()> {
        let header_text = format!("{}-{}", EDITOR_NAME, EDITOR_VERSION);
        let padding = columns.saturating_sub(header_text.len() as u16) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1) as usize);

        Terminal::print(&format!("{}{}{}", SIDE_EDITOR_CHAR, spaces, header_text))?;
        Ok(())
    }

    fn draw_empty_line() -> Result<()> {
        Terminal::print(&format!("{}", SIDE_EDITOR_CHAR))?;
        Ok(())
    }
}
