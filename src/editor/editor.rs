use std::io::Result;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};

use crate::editor::constants::KEYBOARD_CLOSE_SHORTCUT;
use crate::editor::{
    constants::{TERMINAL_INIT_POSITION, TERMINATION_MESSAGE},
    terminal::Terminal,
    view::View,
};

#[derive(Debug)]
pub struct Editor {
    current_position: (u16, u16),
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            should_quit: false,
            current_position: TERMINAL_INIT_POSITION,
            view: View::new(),
        }
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
            self.evaluate_event(&mut event)?;
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
            self.view.render()?;
            Terminal::move_cursor_to(self.current_position)?;
            Terminal::flush()?;
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<()> {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            //Handle for now  code + modifier
            match code {
                KeyCode::Char(KEYBOARD_CLOSE_SHORTCUT) if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Right
                | KeyCode::Left
                | KeyCode::PageUp
                | KeyCode::Home
                | KeyCode::End
                | KeyCode::PageDown => self.move_cursor(*code)?,
                _ => (),
            }
        }
        Ok(())
    }

    fn move_cursor(&mut self, key_code: KeyCode) -> Result<()> {
        let (columns, rows) = Terminal::size()?;
        let (x, y) = self.current_position;

        self.current_position = match key_code {
            KeyCode::Right => {
                let max_x = columns.saturating_sub(1);
                (x.saturating_add(1).min(max_x), y)
            }
            KeyCode::Left => (x.saturating_sub(1), y),
            KeyCode::Down => {
                let max_y = rows.saturating_sub(1);
                (x, y.saturating_add(1).min(max_y))
            }
            KeyCode::Up => (x, y.saturating_sub(1)),
            _ => self.current_position,
        };

        Ok(())
    }
}
